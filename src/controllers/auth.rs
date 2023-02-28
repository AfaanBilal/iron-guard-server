/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use std::time::SystemTime;

use bcrypt::verify;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::*;

use super::{error_response, ErrorResponder};
use crate::{
    entities::{prelude::*, user},
    Config,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: i32,
    role: String,
    exp: u64,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestSignIn<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseSignIn {
    pub status: String,
    pub token: String,
}

pub struct AuthenticatedUser {
    pub role: Role,
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("token") {
            let config = req.rocket().state::<Config>().unwrap();

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
            };

            match Role::from_str(&claims.role) {
                Role::User => Outcome::Success(AuthenticatedUser {
                    role: Role::User,
                    id: claims.sub,
                }),
                Role::Admin => Outcome::Success(AuthenticatedUser {
                    role: Role::Admin,
                    id: claims.sub,
                }),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    config: &State<Config>,
    req_sign_in: Json<RequestSignIn<'_>>,
) -> Result<Json<ResponseSignIn>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let u: user::Model = match User::find()
        .filter(user::Column::Email.contains(req_sign_in.email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(error_response(
                Status::Unauthorized,
                "Invalid credentials".to_string(),
            ))
        }
    };

    if !verify(req_sign_in.password, &u.password).unwrap() {
        return Err(error_response(
            Status::Unauthorized,
            "Invalid credentials".to_string(),
        ));
    }

    let claims = Claims {
        role: u.role,
        sub: u.id,
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .unwrap();

    Ok(Json(ResponseSignIn {
        status: "success".to_string(),
        token,
    }))
}
