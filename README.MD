# Rust Backend with Diesel (SQLite)

This is a Rust backend project using [Diesel ORM](https://diesel.rs/) and SQLite as the database. It provides a foundation for performing basic CRUD operations (Create, Read, Update, Delete) via the command-line interface or server-side logic.

---

## Prerequisites

- [Rust (1.88.0)](https://rust-lang.org)
- SQLite installed
- Diesel CLI
- [Make sure to install `libsqlite3` if you faced problems with migrations]

---

## Setup Instructions

### 1. Install Diesel CLI
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

### 2. Install Diesel CLI
```env
DATABASE_URL=db.sqlite
```

### 3. Initialize Diesel and run migrations
```env
diesel setup
diesel migration run
```

### 4. Run Diesel Migrations:
```bash
diesel setup
diesel migration run
```

### 5. Update id in the schema.rs manually
```bash
id -> Integer,
```

### 5. Build and run
```bash
cargo run
```
