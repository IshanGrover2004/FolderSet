use actix_web::{
   dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
   Error as ActixWebError, HttpMessage, HttpResponse, error::InternalError,
};
use futures_util::future::{LocalBoxFuture, ready, Ready};
use crate::jwt::verify_jwt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
   S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
   S::Future: 'static,
   B: 'static,
{
   type Response = ServiceResponse<B>;
   type Error = ActixWebError;
   type InitError = ();
   type Transform = AuthMiddlewareService<S>;
   type Future = Ready<Result<Self::Transform, Self::InitError>>;

   fn new_transform(&self, service: S) -> Self::Future {
       ready(Ok(AuthMiddlewareService { service }))
   }
}

pub struct AuthMiddlewareService<S> {
   service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
   S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
   S::Future: 'static,
   B: 'static,
{
   type Response = ServiceResponse<B>;
   type Error = ActixWebError;
   type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

   forward_ready!(service);

   fn call(&self, req: ServiceRequest) -> Self::Future {
      let token = req.headers().get("Authorization").and_then(|header| {
          header.to_str().ok().and_then(|s| {
              s.strip_prefix("Bearer ").map(|t| t.to_string())
          })
      });
  
      if let Some(token) = token {
          match verify_jwt(&token) {
              Ok(claims) => {
                  req.extensions_mut().insert(claims.sub);
                  let fut = self.service.call(req);
                  Box::pin(async move {
                      let res = fut.await?;
                      Ok(res)
                  })
              }
              Err(_) => {
               eprintln!("Token verification failed for token: {}", token);
                  let response = HttpResponse::Unauthorized().finish();
                  Box::pin(async { Err(InternalError::from_response("", response).into()) })
              }
          }
      } else {
          eprintln!("No token found in Authorization header");
          let response = HttpResponse::Unauthorized().finish();
          Box::pin(async { Err(InternalError::from_response("", response).into()) })
      }
  }
}
