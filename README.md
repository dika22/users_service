# User Service

A RESTful API for managing users, built with Rust, Axum, and SQLx.

## Tech Stack

-   **Language:** Rust (2024 edition)
-   **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
-   **Database:** PostgreSQL
-   **ORM/Query Builder:** [SQLx](https://github.com/launchbadge/sqlx)
-   **Runtime:** [Tokio](https://tokio.rs/)
-   **Serialization:** Serde

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   [PostgreSQL](https://www.postgresql.org/) running locally or accessible via URL.

## Setup

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>
    cd user_service
    ```

2.  **Environment Configuration:**

    Create a `.env` file in the root directory and set your `DATABASE_URL`:

    ```bash
    DATABASE_URL=postgres://postgres:postgres@localhost/mydb
    ```

    Replace the credentials (`postgres:postgres`) and database name (`mydb`) with your actual PostgreSQL configuration.

3.  **Database Setup:**

    Ensure your database exists. You can use `sqlx-cli` to manage migrations if you have it installed, or create the table manually:

    ```sql
    CREATE TABLE users (
        id UUID PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE
    );
    ```

## Running the Application

Run the server with cargo:

```bash
cargo run
```

The server will start at `http://0.0.0.0:3000`.

## API Endpoints

### Users

| Method | Endpoint     | Description           | Request Body          |
| :----- | :----------- | :-------------------- | :-------------------- |
| `GET`  | `/users`     | Get all users         | -                     |
| `POST` | `/users`     | Create a new user     | `{ "name": "...", "email": "..." }` |
| `GET`  | `/users/:id` | Get a user by ID      | -                     |
| `DELETE`| `/users/:id`| Delete a user by ID   | -                     |

### Example Requests

**Create User:**

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

**Get All Users:**

```bash
curl http://localhost:3000/users
```

## Project Structure

```
src/
├── handlers/       # HTTP request handlers
├── models/         # Data structures and DTOs
├── repository/     # Database access layer
├── services/       # Business logic
├── routes.rs       # Route definitions
├── state.rs        # Application state (DB pool)
└── main.rs         # Entry point
```
