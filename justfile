build:
    cargo build

lint:
    cargo fmt

test:
    cargo test

run-debug: build
    ./target/debug/sample_project

run-release:
    cargo build --release
    ./target/release/sample_project

create-migration name:
    sqlx migrate add -r {{name}}

# -------------------- database -------------------- #
db := env_var_or_default("DB", "sample_project.db")

db:
    sqlite3 {{db}}

db-tables:
    sqlite3 {{db}} ".tables"

db-reset:
    rm -f {{db}} {{db}}-wal {{db}}-shm
    @echo "removed {{db}}"
