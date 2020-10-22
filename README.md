# Food Organizer


## Description

This is an API to store personal food data.

**This is a work in progress**

## Requirements

1. Docker
2. Docker Compose
3. [Rust Stable](https://www.rust-lang.org/tools/install)
4. [Cargo Make](https://github.com/sagiegurari/cargo-make) (to run tests)
5. [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) (to run diesel's migrations)

## Steps to run

1. Run `docker compose up -d` to start Postgres Database
2. Run `cargo run` to run Actix-Web API

### Routes

*GET* - http://localhost:3000/api/recipes
*POST* - http://localhost:3000/api/recipes

Example of json body:

```json
{
    "name": "Ham Sandwich",
    "description": "Something"
}
```

## How to run tests

To run tests run the following command: `cargo make --makefile tasks.toml test-flow`
