# Login service

Rust microservice to authenticate users

## Setup

[Install Rust](https://www.rust-lang.org/tools/install)

Install Postgres

Create .env file

| Variable name        | Example                                                     |
|----------------------|-------------------------------------------------------------|
| DATABASE_URL         | postgresql://username:password@localhost:5432/login_service |
| SECURITY_SALT        | kjdjkhnsdkjfhsjkdhkjasdhkjdshf                              |
| JWT_SALT             | sddladkjsdfkjsdlfjsdfj                                      |
| RABBITMQ_URI         | amqp://guest:guest@localhost:5672/                          |
| BACKEND_CORS_ORIGINS | []                                                          |

```
cargo run
```

Create director

```
cargo run create_director director@pogchamp.ru qwerty
```

To change address, port, log_level edit Rocket.toml file

## Code guidelines

### Use clippy

#### Run clippy

```
cargo clippy -- -A clippy::needless_return
```

clippy::needless_return should be ignored.
As omitted "return" in medium/large functions does not do anything good, but confuse people from other languages.

Use clippy to catch all inconveniences

### Run tests before committing

```
cargo test
```

## [RabbitMQ consumers documentation](src/consumers/README.md)