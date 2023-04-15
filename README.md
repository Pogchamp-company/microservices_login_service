# Login service
Rust microservice to authenticate users

## Setup
[Install Rust](https://www.rust-lang.org/tools/install)

Install Postgres

Create .env file

| Variable name | Example                                                     |
|---------------|-------------------------------------------------------------|
| DATABASE_URL  | postgresql://username:password@localhost:5432/login_service |
| SECURITY_SALT | kjdjkhnsdkjfhsjkdhkjasdhkjdshf                              |
| JWT_SALT      | sddladkjsdfkjsdlfjsdfj                                      |

```
cargo run
```
