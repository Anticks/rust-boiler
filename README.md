# rust-boiler

A boilerplate Rust project to kickstart development of new applications with common best practices and project structure.

## Features

- Pre-configured Rust workspace with `Cargo.toml` and `Cargo.lock`
- Docker support (`Dockerfile`, `.dockerignore`)
- Environment and configuration files (`configuration.yaml`, `config.json`)
- Database schema and migration support (`schema.sql`, `migrations/`, `.sqlx/`)
- Example scripts and test structure (`scripts/`, `tests/`)
- Organized source code in `src/`
- Infrastructure and deployment hints (`atlas.hcl`)

## Structure

```
.
├── .dockerignore
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── atlas.hcl
├── config.json
├── configuration.yaml
├── schema.sql
├── migrations/
├── scripts/
├── src/
├── tests/
└── .sqlx/
```

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (nightly or stable)
- [Docker](https://www.docker.com/) (optional, for containerized development)
- (Optional) SQL database compatible with the provided schema

### Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/Anticks/rust-boiler.git
   cd rust-boiler
   ```

2. **Install dependencies:**
   ```sh
   cargo build
   ```

3. **Run the project:**
   ```sh
   cargo run
   ```

4. **To run tests:**
   ```sh
   cargo test
   ```

### Using Docker

To build and run using Docker:

```sh
docker build -t rust-boiler .
docker run --rm rust-boiler
```

## Configuration

- `configuration.yaml` and `config.json` provide customizable settings.
- SQL schema and migration files are located in `schema.sql` and `migrations/`.

## Contributing

Feel free to fork, open issues, and submit pull requests to improve this boilerplate.

## License

This project is provided as-is. Please add a license if you intend to distribute or use it broadly.

---

> Generated with ❤️ by [Anticks](https://github.com/Anticks)
