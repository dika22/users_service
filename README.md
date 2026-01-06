# User Service

A RESTful API for managing users, built with Rust, Axum, and SQLx.

## Tech Stack

-   **Language:** Rust (2024 edition)
-   **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
-   **Database:** PostgreSQL
-   **ORM/Query Builder:** [SQLx](https://github.com/launchbadge/sqlx)
-   **Runtime:** [Tokio](https://tokio.rs/)
-   **Serialization:** Serde
-   **Documentation:** [Utoipa](https://github.com/juhaku/utoipa) (Swagger UI)
-   **Security:** Bcrypt (Password Hashing)

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   [PostgreSQL](https://www.postgresql.org/) running locally or accessible via URL.
-   [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli` (Recommended for migrations)

## Setup

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>
    cd user_service
    ```

2.  **Environment Configuration:**

    Create a `.env` file in the root directory:

    ```bash
    DATABASE_URL=postgres://postgres:postgres@localhost/mydb
    RUST_LOG=debug
    PORT=3001
    HOST=0.0.0.0
    ```

3.  **Database Setup:**

    Run database migrations to set up the schema (including `users` and `users_auth` tables):

    ```bash
    sqlx database create
    sqlx migrate run
    ```

## Running the Application

Run the server with cargo:

```bash
cargo run
```

The server will start at `http://0.0.0.0:3001`.

## API Documentation

Interactive API documentation is available via Swagger UI:

-   **Swagger UI:** [http://localhost:3001/swagger-ui/](http://localhost:3001/swagger-ui/)
-   **OpenAPI Spec:** [http://localhost:3001/api-docs/openapi.json](http://localhost:3001/api-docs/openapi.json)

## API Endpoints

### Auth & Users

| Method | Endpoint | Description | Request Body |
| :--- | :--- | :--- | :--- |
| `POST` | `/users/register` | Register a new user | `{ "username": "...", "email": "...", "password": "...", "created_by": "..." }` |
| `POST` | `/users/login` | Login user | `{ "email": "...", "password": "..." }` |
| `GET` | `/users` | Get all users | - |
| `POST` | `/users` | Create a simple user (Legacy) | `{ "name": "...", "email": "..." }` |
| `GET` | `/users/:id` | Get a user by ID | - |
| `PUT` | `/users/:id` | Update a user by ID | `{ "name": "...", "email": "..." }` |
| `DELETE` | `/users/:id` | Delete a user by ID | - |

### Example Requests

**Register User:**

```bash
curl -X POST http://localhost:3001/users/register \
  -H "Content-Type: application/json" \
  -d '{"username": "johndoe", "email": "john@example.com", "password": "securepassword", "created_by": "system"}'
```

**Login User:**

```bash
curl -X POST http://localhost:3001/users/login \
  -H "Content-Type: application/json" \
  -d '{"email": "john@example.com", "password": "securepassword"}'
```

## Project Structure

```
src/
├── docs.rs         # Swagger/OpenAPI configuration
├── handlers/       # HTTP request handlers
├── models/         # Data structures and DTOs
├── repository/     # Database access layer
├── services/       # Business logic
├── routes.rs       # Route definitions
├── state.rs        # Application state (DB pool)
└── main.rs         # Entry point
```
