### Migrations

Migrations are run when the application starts.

```
cargo install sqlx-cli
export DATABASE_URL=sqlite://sqlite.db
sqlx database create
sqlx migrate add init
sqlx migrate run
```
