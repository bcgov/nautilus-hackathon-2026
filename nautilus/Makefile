.PHONY: deps run db-install db-cli db-migrate db-setup db-reset db-clean db-seed

deps:
	cargo add rocket --features json
	cargo add sqlx --features macros,migrate,sqlite
	cargo add anyhow

run:
	cargo run

db-install:
	brew install sqlite

db-cli:
	cargo install sqlx-cli --no-default-features --features sqlite

db-migrate:
	DATABASE_URL=sqlite://nautilus.db cargo sqlx migrate run

db-setup: db-install db-cli db-migrate

db-clean:
	rm -f nautilus.db

db-reset: db-clean db-migrate

db-seed:
	sqlite3 nautilus.db < seeds/0001_seed.sql
