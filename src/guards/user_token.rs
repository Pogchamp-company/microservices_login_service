use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::user::load_user;
use crate::models::user_role::UserRole;
use crate::password_utils::get_email_from_token;

#[derive()]
pub struct UserTokenInfo {
    pub email: String,
    pub roles: Vec<UserRole>,
}


#[derive(Debug, Serialize)]
pub enum UserTokenError {
    Missing { message: String },
    BadCount { message: String },
    Parse { message: String },
    EmailNotFound { message: String },
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserTokenInfo {
    type Error = UserTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let tokens: Vec<_> = request.headers().get("Authorization").collect();
        if tokens.is_empty() {
            return Outcome::Failure((Status::Unauthorized, UserTokenError::Missing {
                message: "Токен не предоставлен".to_string()
            }));
        }

        if tokens.len() > 1 {
            return Outcome::Failure((Status::Unauthorized, UserTokenError::BadCount {
                message: "Несколько токенов".to_string()
            }));
        }

        let token = tokens[0];

        let email = get_email_from_token(token);

        let email = match email {
            Ok(email) => email,

            Err(error_message) => {
                return Outcome::Failure((Status::Unauthorized, UserTokenError::Parse {
                    message: error_message,
                }));
            }
        };

        let pool = request.rocket().state::<PgPool>().unwrap();

        let user = load_user(&email, pool).await;

        let user = match user {
            Ok(user) => user,

            Err(error_message) => {
                return Outcome::Failure((Status::Unauthorized, UserTokenError::EmailNotFound {
                    message: error_message,
                }));
            }
        };

        return Outcome::Success(UserTokenInfo {
            email,
            roles: user.roles,
        });
    }
}

impl OpenApiFromRequest<'_> for UserTokenInfo {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires an User Token to access.".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("UserTokenAuth".to_owned(), Vec::new());

        Ok(RequestHeaderInput::Security(
            "UserTokenAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}