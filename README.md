juniper-sqlx-sandbox
====================

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
    - Required version: 1.67.0 or later
1. Install PostgreSQL
    - Required version: 14.0 or later
1. Setup [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
    - `cargo install sqlx-cli`
1. Clone this repository
1. Create `.env` file
    - `cp .env.example .env`
1. Edit `.env` file
    - `DATABASE_URL` is the connection string to the PostgreSQL database.
    - Now the default value is `postgres://postgres:postgres@localhost:5432/juniper-sqlx`.
1. Run `sqlx database create`
    - This command creates a database named `juniper-sqlx`.
1. Run `sqlx migrate run`
    - This command creates table named `actors` and `posts` tables.
1. Add some data to the database.
1. Run `cargo run`
    - This command starts a GraphQL server.
1. Open http://localhost:8082/graphiql in your browser.
    - You can use GraphiQL to execute GraphQL queries.
