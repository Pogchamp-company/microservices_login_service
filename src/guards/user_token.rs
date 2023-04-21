use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};
use sqlx::PgPool;
use crate::models::user::{load_user};
use crate::models::user_role::UserRole;
use crate::password_utils::get_email_from_token;

pub struct UserTokenInfo {
    pub email: String,
    pub roles: Vec<UserRole>,
}

#[derive(Debug)]
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
        if tokens.len() == 0 {
            return Outcome::Failure((Status::BadRequest, UserTokenError::Missing));
        }

        if tokens.len() > 1 {
            return Outcome::Failure((Status::BadRequest, UserTokenError::BadCount));
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
