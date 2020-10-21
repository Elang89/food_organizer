# Food Organizer


## Description

This is an API to store personal food data.

**This is a work in progress**

## Requirements

1. Docker
2. Docker Compose
3. Rust Stable
4. Cargo Make (to run tests)

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
