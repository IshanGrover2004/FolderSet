# FolderSet Web Application

FolderSet is a web application built with Rust and Actix-web for managing folders and files.

## Prerequisites

Before running the application, ensure you have the following installed:

- Rust (stable)
- Cargo (Rust's package manager)
- Docker

## Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/folderset.git
   cd folderset

   ```

2. **Set up environment variables:**
   Create a .env file in the root directory of the project and add the necessary environment variables:

   ```bash
   DATABASE_URL=postgres://myuser:mysecretpassword@localhost/mydatabase
   ```

   Replace myuser, mysecretpassword, and mydatabase with the credentials and database name specified in your Docker Compose file.

3. **Start PostgreSQL with Docker Compose:**
   Run Docker Compose to start PostgreSQL and create a persistent volume for data:

   ````bash
   docker-compose up -d
   ````
   This command starts PostgreSQL in the background using the configuration specified in your docker-compose.yml file.

4. **Install dependencies:**

   ```bash
   cargo build
   ```

5. **Run the migrations:**
   Ensure your database schema is up-to-date by running Diesel migrations:
   ```bash
   diesel migration run
   ```
6. **Start the application:**

   ```bash
      cargo run
   ```

7. **Access the application:**

   Open your web browser and go to http://127.0.0.1:8080 to access the FolderSet application.
