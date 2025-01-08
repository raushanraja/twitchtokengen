# Auther

This project is a Rust-based web server using Actix-web, designed to interact with the Twitch API for OAuth2 authentication and store user tokens in a PostgreSQL database.

## Project Structure

```
.
├── .env
├── .github/
│   └── workflows/
│       └── rust.yml
├── .gitignore
├── apitest/
│   └── generatetoken.http
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
├── src/
│   ├── api/
│   │   ├── app_data.rs
│   │   ├── db/
│   │   │   ├── client.rs
│   │   │   └── mod.rs
│   │   ├── mod.rs
│   │   └── v1/
│   │       ├── endpoints/
│   │       │   └── auth.rs
│   │       └── mod.rs
│   ├── config/
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── target/
```

## Getting Started

### Prerequisites

- Rust (1.83.0 or later)
- PostgreSQL
- Docker (optional)

### Environment Variables

Create a `.env` file in the root directory with the following variables:

```
TWITCH_CLIENT_ID=<your_twitch_client_id>
TWITCH_CLIENT_SECRET=<your_twitch_client_secret>
TWITCH_REDIRECT_URL=<your_twitch_redirect_url>
SUPABASE_URL=<your_supabase_url>
SUPABASE_SECRET_KEY=<your_supabase_secret_key>
PORT=8080
```

### Building and Running
To build and run the project locally:

```sh
cargo build
cargo run
```

### Using Docker

To build and run the project using Docker:

```sh
docker build -t api_starter .
docker run --env-file .env -p 8080:8080 api_starter
```

### API Endpoints

- `GET /generatetoken`: Generates a Twitch OAuth2 token URL and stores the CSRF token in a cache.
- `GET /`: Callback endpoint for Twitch OAuth2, retrieves the user token and stores it in the PostgreSQL database.


### GitHub Actions
The project uses GitHub Actions for CI/CD. The workflow is defined in `.github/workflows/rust.yml`.
