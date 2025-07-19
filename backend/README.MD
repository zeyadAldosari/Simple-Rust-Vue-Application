# Backend

This is the backend for the project, built with Rust, Axum, Diesel ORM, and SQLite.

## Project Structure

- `src/` – Main source code
  - `main.rs` – API server entry point (Axum)
  - `handlers.rs` – HTTP request handlers (register, login)
  - `models.rs` – Data models and validation
  - `db.rs` – Database connection logic
  - `schema.rs` – Diesel schema (auto-generated)
  - `utils.rs` – Utility functions (e.g., password hashing)
  - `bin/` – CLI utilities (create, get, show, delete users)
- `migrations/` – Database migrations (users table)
- `Cargo.toml` – Rust dependencies and metadata
- `diesel.toml` – Diesel configuration

## Prerequisites
- [Rust](https://rust-lang.org) (1.88.0 or newer)
- SQLite
- Diesel CLI (`cargo install diesel_cli --no-default-features --features sqlite`)
- `libsqlite3` (for migrations)

## Setup Instructions

1. Set the database URL:
   ```env
   export DATABASE_URL=db.sqlite
   ```
2. Run Diesel migrations:
   ```bash
   diesel setup
   diesel migration run
   ```
3. Build and run the server:
   ```bash
   cargo run
   ```
4. Update the schema.rs manually if the id not Interger (sqlite metadata limitation)

## API Endpoints

- `POST /api/register` – Register a new user
  - Request: `{ "username": "email", "password": "string" }`
  - Password must be at least 8 characters, with uppercase, lowercase, digit, and special character
  - Response: Success or error message

- `POST /api/login` – Login with username and password
  - Request: `{ "username": "email", "password": "string" }`
  - Response: Success or error message

## Tech Stack
- [Rust](https://rust-lang.org/)
- [Axum](https://docs.rs/axum/)
- [Diesel ORM](https://diesel.rs/)
- [SQLite](https://www.sqlite.org/)
- [Tokio](https://tokio.rs/)

## Notes
- The backend listens on `0.0.0.0:3000` by default.
- CORS is enabled for `http://localhost:5173` (the frontend dev server).
- See the main project README for frontend setup and integration.
