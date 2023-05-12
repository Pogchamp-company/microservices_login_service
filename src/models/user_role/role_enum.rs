use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use rocket::request::FromParam;
use rocket::serde::json::serde_json;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone, JsonSchema, Eq, Hash, Copy)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    TaskManager,
    HumanResources,
    Director,
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_user_role")
    }
}

impl<'a> FromParam<'a> for UserRole {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let enum_value = serde_json::from_str::<UserRole>(&format!(r#""{}""#, param));
        match enum_value {
            Ok(role) => {
                Ok(role)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(param)
            }
        }
    }
}
