# DigiTour backend

<!-- toc -->

- [Installation](#installation)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
- [Usage](#usage)

<!-- tocstop -->

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Docker Compose](https://docs.docker.com/compose/)
 
### Steps

1. Clone the repository and `cd` into it.
2. Create local database.
```sh
docker-compose up -d
```
3. Set the environment variables by copying `.env.example` to `.env`.
4. Install [`sqlx-cli`](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).
```sh
cargo install sqlx-cli
```
5. Make migrations.
```sh
cargo sqlx migrate run
```
6. Run the server.
```sh
cargo run
```
7. (optional) Install [`cargo-watch`](https://crates.io/crates/cargo-watch) to automatically recompile after file change.
```sh
cargo install cargo-watch
```
Run `cargo-watch`.
```sh
cargo watch -x run
```

## Usage
After following the installation steps, you can access the application at [localhost:8080](http://localhost:8080).
