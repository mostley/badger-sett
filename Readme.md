# Bader Sett
small api to access the badger db

## Setup
    cargo install
    cargo install sqlx-cli --no-default-features --features sqlite

## Update sqlx query definitions
This is needed to update the typing information for the query strings

    DATABASE_URL="sqlite:$(pwd)/badger.db" cargo sqlx prepare

## Usage
To start the server run

    cargo run
