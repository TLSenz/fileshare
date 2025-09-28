# Fileshare API

## 🚀 Overview

Welcome to the Fileshare API! This repository contains a small file sharing/back‑end service built in Rust using Axum and Tokio. It provides endpoints to sign up and log in users, upload files, and download them back via generated links. Files are currently stored on the local filesystem (content/ directory) and metadata is stored in a Postgres database via SQLx.

The existing roadmap below is kept as context for future directions. This README focuses on how to run what exists today.

## 🧰 Tech Stack

- Language: Rust (edition 2024)
- Web framework: Axum 0.8
- Async runtime: Tokio 1.x (multi-thread)
- Database: PostgreSQL (via SQLx 0.8)
- Auth: JWT (jsonwebtoken)
- HTTP utilities: tower-http
- Env/config: dotenv
- Optional (present but currently unused in code path): AWS SDK for S3 (upload code exists but is commented out)

Package manager and build tool: Cargo

## 📦 Entry Points and Binaries

- Binary: fileshare (defined in Cargo.toml)
  - Entry point: src/main.rs
  - It calls startup() from src/startup.rs to build the Axum router and start the server
- Library crate: src/lib.rs re-exports modules used by the binary and tests

## 🔌 HTTP API (current)

Server binds to: 0.0.0.0:3000

Mounted routes (from src/startup.rs):
- GET / → health_check (200 OK)
- POST /api/signup → create a new user (JSON body: { name, email, password })
- POST /api/login → returns JWT on valid credentials (JSON body: { name, email, password })
- POST /api/upload → multipart upload of one or more files; requires Bearer token; stores to content/ and DB; returns a download link
- GET /api/download/{file_link} → downloads the file by hashed link
- Static files: /files → serves from local content/ directory

Auth for protected routes:
- Authorization: Bearer <jwt>
- JWT is signed/verified with the secret in env var JWT_SECRET and includes exp (24h). Middleware checks the user exists in DB.

## 📁 Project Structure

- src/
  - main.rs (binary entry)
  - lib.rs (module exports)
  - startup.rs (router + server bind)
  - db.rs (PgPool init via DATABASE_URL)
  - routes/
    - healthc_check.rs (health_check)
    - signup.rs (signup endpoint)
    - login.rs (login endpoint)
    - upload.rs (multipart upload; local file write; link generation)
    - download.rs (download by link)
  - repository/
    - userrepository.rs (users CRUD/checks)
    - filerepository.rs (files CRUD/queries)
  - model/
    - usermodel.rs (User, SignupRequest, LoginRequest, LoginResponse, File, FileToInsert, ConversionError)
    - filemodel.rs (GetFileResponse)
    - securitymodel.rs (AuthError, EncodeJWT)
  - security/
    - jwt.rs (encode/decode JWT + authenticate middleware)
  - service/ (present; internal services, if any; TODO verify usage)
- migrations/ (*.sql for users, file, file_to_link tables)
- content/ (local storage directory for uploaded files; is served via /files)
- tests/ (integration tests)
- docker-compose.yaml (Postgres + Adminer)
- flake.nix (Nix dev shell with rust, cargo, openssl, pkg-config, docker-compose, sqlx-cli)

## 🧪 Tests

- Integration test: tests/login.rs posts to http://127.0.0.1:3000/api/login and expects a token.
  - Important: This test expects the server to be running and a matching user existing in the DB. Otherwise it will fail trying to connect or validate credentials.

Run tests:
- Start the server (see Run locally below)
- Ensure the DB has a user matching the credentials used in the test ("Test"/"Test"/"test@test.email") or adjust the test
- cargo test

Note: There is also a helper binary test_connection.rs that can be run with cargo run --bin test_connection if added to Cargo.toml; currently it is just a standalone file and not configured as a Cargo bin. TODO: Decide whether to remove it or register it as an auxiliary binary.

## 🔧 Requirements

- Rust toolchain (rustup, rustc, cargo)
- OpenSSL and pkg-config (required for SQLx on many systems)
- PostgreSQL instance
- Optionally Docker (to run Postgres via docker-compose)
- Optionally Nix (to use the provided flake for setting up the environment)

## ⚙️ Environment Variables

- DATABASE_URL: Postgres connection string. Example:
  - postgres://postgres:example@localhost:5432/postgres
- JWT_SECRET: secret used to sign/verify JWTs
- Optional (only if enabling S3 upload path in routes/upload.rs):
  - AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION (and bucket name inside code)

Place them in a .env file for local development (dotenv is loaded at runtime):

DATABASE_URL=postgres://postgres:example@localhost:5432/postgres
JWT_SECRET=change_me

## 🗄️ Database and Migrations

- Migrations are under migrations/ and target PostgreSQL.
- Use sqlx-cli to run them:
  - Install: cargo install sqlx-cli --no-default-features --features native-tls,postgres
  - Run: sqlx migrate run

The docker-compose.yaml provides a basic Postgres (password example) and Adminer UI on port 80.

Start with Docker:
- docker compose up -d
- Create .env with DATABASE_URL and JWT_SECRET
- Run migrations: sqlx migrate run

## ▶️ Run locally

Using Cargo:
- Ensure DATABASE_URL and JWT_SECRET are set (or present in .env)
- Ensure the content/ directory exists and is writable (created in repo).
- cargo run
- Server logs: Server running on http://0.0.0.0:3000

Using Nix dev shell (optional):
- nix develop
- Inside the shell you have cargo, openssl, pkg-config, docker-compose, sqlx-cli

Using Docker for DB only (app runs on host):
- docker compose up -d
- cargo run

## 📜 Scripts and Commands

- Run app: cargo run
- Run tests: cargo test (requires server running for integration tests)
- Run migrations: sqlx migrate run
- Start DB + Adminer: docker compose up -d
- Stop DB + Adminer: docker compose down
- Enter Nix dev shell: nix develop

## 🔐 Upload/Download behavior

- Upload (POST /api/upload):
  - Requires Authorization: Bearer <jwt>
  - Accepts multipart files. Each file is written to content/<original_name>.<ext>
  - A hashed link (bcrypt-derived) is stored with metadata; response returns a link like http://127.0.0.1:3000/api/download/<hash>
  - TODO: Collision/duplicate handling and validation can be improved; currently check_if_file_name_exists returns true when the name does not exist.
- Download (GET /api/download/{file_link}):
  - Resolves the hashed link to storage_path and streams the file with a guessed Content-Type.

## 🧭 Roadmap (kept)

[The original multi-phase roadmap is preserved here for context and planning.]

### Phase 1: Core System & Data Foundation

- Folder management
- File rename/delete
- Basic metadata
- Admin user management

Key Rust topics: structs/enums, ownership/borrowing/lifetimes, Result/?, collections, traits.

### Phase 2: Secure Sharing & Versioning

- Shareable links with controls (passwords, expiry, limits)
- Versioning
- Storage quotas

Key Rust topics: generics, concurrency primitives, chrono, crypto crates, iterators.

### Phase 3: Eventing, Security & Scalability

- Webhooks
- Audit logs
- Rate limiting
- 2FA/MFA
- AV scanning

Key Rust topics: async, channels, DB clients, reqwest, error handling.

### Phase 4: Enhanced UX & Analytics

- Previews, search, analytics, etc.

## 📄 License

TODO: Add a LICENSE file (e.g., MIT/Apache-2.0) and reference it here.
