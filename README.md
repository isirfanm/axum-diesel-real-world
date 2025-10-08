# Axum Diesel Real World

Article: https://medium.com/@qkpiot/building-a-robust-rust-backend-with-axum-diesel-postgresql-and-ddd-from-concept-to-deployment-b25cf5c65bc8 
GitHub: https://github.com/Quentin-Piot/axum-diesel-real-world

## PostgreSQL

Install UUID extension:

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

## Diesel

Install Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features "postgres"
```

Setup PostgreSQL connection configuration in .env file:

```bash
DATABASE_URL=postgres://$USER:$PASSWORD@localhost/$DATABASE
```

Setup diesel migration:

```bash
diesel setup
```

Comfigure diesel setting in diesel.toml file:

```toml
[print_schema]
file = "src/infra/db/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId"]

[migrations_directory]
dir = "migrations"
```

Create new migration:

```bash
diesel migration generate create_posts
```

This will create new migration 'create_posts` inside migration folder. Edit up.sql and down.sql files inside the new migration. 

Example up.sql :

```
CREATE TABLE posts
(
    id        uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
```

Example down.sql :

```
DROP TABLE posts
```

Apply migration to database:

```bash
diesel migration run
```

A new file will be created by diesel inside schema module (schema.rs):

```rust
diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

## Testing

```bash
# create a new post
curl -X POST -H "Content-Type: application/json" -d '{"title":"first post", "body":"this is the body of my post"}' localhost:3000/v1/posts
# {"id":"8313961e-7caf-48a6-8ba0-bd9ff0b60953","title":"first post","body":"this is the body of my post","published":false}

# get a specific post by id
curl -X GET localhost:3000/v1/posts/8313961e-7caf-48a6-8ba0-bd9ff0b60953
# {"id":"8313961e-7caf-48a6-8ba0-bd9ff0b60953","title":"first post","body":"this is the body of my post","published":false}

# get all posts
curl -X GET localhost:3000/v1/posts
# {"posts":[{"id":"8313961e-7caf-48a6-8ba0-bd9ff0b60953","title":"first post","body":"this is the body of my post","published":false}]}

# update a specific post
curl -v -X PUT -H "Content-Type: application/json" -d '{"published":true}' localhost:3000/v1/posts/8313961e-7caf-48a6-8ba0-bd9ff0b60953
# {"id":"8313961e-7caf-48a6-8ba0-bd9ff0b60953","title":"first post","body":"this is the body of my post","published":true}
```
