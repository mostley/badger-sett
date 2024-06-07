cargo install sqlx-cli --no-default-features --features sqlite
DATABASE_URL="sqlite:$(pwd)/badger.db" cargo sqlx prepare
