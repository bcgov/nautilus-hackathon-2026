.PHONY: deps run watch db-install db-cli db-migrate db-setup db-reset db-clean db-seed

deps:
	cargo add rocket --features json
	cargo add rocket_dyn_templates --features tera
	cargo add sqlx --features macros,migrate,sqlite
	cargo add anyhow
	cargo add reqwest --features blocking,json
	cargo add serde --features derive
	cargo add serde_json
	cargo add tokio --features full
	cargo install cargo-watch

run:
	cargo run

watch:
	cargo watch -x "run -- api"

db-install:
	brew install sqlite

db-cli:
	cargo install sqlx-cli --no-default-features --features sqlite

db-migrate:
	DATABASE_URL=sqlite://nautilus.db cargo sqlx migrate run

db-setup: db-install db-cli db-migrate

db-clean:
	rm -f nautilus.db

db-reset: db-clean db-touch db-migrate

db-seed:
	sqlite3 nautilus.db < seeds/0001_seed.sql

db-touch:
	touch nautilus.db
