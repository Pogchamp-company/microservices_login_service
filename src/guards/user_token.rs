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
    Missing,
    BadCount,
    Parse,
    EmailNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserTokenInfo {
    type Error = UserTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let tokens: Vec<_> = request.headers().get("x-user-token").collect();
        if tokens.is_empty() {
            return Outcome::Failure((Status::Unauthorized, UserTokenError::Missing));
        }

        if tokens.len() > 1 {
            return Outcome::Failure((Status::Unauthorized, UserTokenError::BadCount));
        }

        let token = tokens[0];

        let email = get_email_from_token(token);

        let email = match email {
            Ok(email) => email,

            Err(..) => {
                return Outcome::Failure((Status::Unauthorized, UserTokenError::Parse));
            }
        };

        let pool = request.rocket().state::<PgPool>().unwrap();

        let user = load_user(&email, pool).await;

        let user = match user {
            Ok(user) => user,

            Err(..) => {
                return Outcome::Failure((Status::Unauthorized, UserTokenError::EmailNotFound));
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
                name: "x-user-token".to_owned(),
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