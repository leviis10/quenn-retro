# Queen Retro Backend

This is a sprint retrospective platform inspired by [EasyRetro](https://easyretro.io/) written in rust. The project is
already following the rust idiom and best practice as much as possible. The code in this project is even cleaner rather
than my first web application rust project [old-money](https://github.com/leviis10/old-money), but it has fewer
features (there are no swagger docs and Request validation)

## ERD

![Queen Retro ERD](assets/queen-retro.png)

## Tech Stack

- [axum](https://docs.rs/axum/latest/axum/), The Web Framework Core
- [SeaORM](https://www.sea-ql.org/SeaORM/), The ORM

## Features

- Tracing
- Environment Variables Management
- CORS Management
- Graceful Shutdown
- Standardize Success, Paginated, and Error Response
- Database Migration
- Proper Error Handling
- Actuator Endpoint
- Cookie Management
- Response Compression
- Handle Timeout

## How To Run

This application is dockerized and pushed into the docker hub with the name of `leviis/queen-retro`. You can either
build this application using docker build command or pull the image from docker hub. Make sure that the database is
already migrated. You can run use `sea-orm-cli` or `cargo` command to run the migration first

### Docker Run Command Example

```bash
docker container run \
-e PORT=8080 \
-e DB_HOST=<db-host> \
-e DB_NAME=<db-name> \
-e DB_USERNAME=<db-username> \
-e DB_PASSWORD=<db-password> \
-e RUST_LOG="info" \
-e ALLOWED_ORIGINS=origin1,origin2,origin3 \
-p 8080:8080 \
-d \
--name queen-retro \
leviis/queen-retro:1.0.0
```
