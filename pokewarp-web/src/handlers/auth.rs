use std::sync::Arc;

use axum::{async_trait, extract::State, http::StatusCode, Json, response::IntoResponse};
use axum_login::{secrecy::{SecretVec, ExposeSecret}, AuthUser, UserStore, extractors::AuthContext};
use axum_macros::debug_handler;
use eyre::ErrReport;
use password_auth::{generate_hash, verify_password, VerifyError};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal, Response};


use crate::AppState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: Thing,
    name: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
struct UserRecord {
    name: String,
    password_hash: String,
}

impl AuthUser<Thing> for User {
    fn get_id(&self) -> Thing {
        self.id.clone()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

#[derive(Clone)]
pub struct SurrealUserStore {
    conn: Arc<Surreal<Client>>,
}

impl SurrealUserStore {
    pub fn new(conn: Arc<Surreal<Client>>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserStore<Thing, ()> for SurrealUserStore {
    type User = User;

    async fn load_user(&self, user_id: &Thing) -> Result<Option<User>, eyre::Error> {
        self.conn
            .select(("user", user_id.clone()))
            .await
            .map_err(|err| eyre::Error::new(err))
    }
}

#[derive(Deserialize)]
pub struct SignupReq {
    name: String,
    password: String,
}

type LoginReq = SignupReq;  // In the future we may want to differentiate between Login and Signup

pub enum AuthError {
    SurrealError(surrealdb::Error),
    EyreReport(eyre::Report),
    VerifyError(VerifyError),
    NoUser
}

impl From<surrealdb::Error> for AuthError {
    fn from(value: surrealdb::Error) -> Self {
        AuthError::SurrealError(value)
    }
}

impl From<eyre::Report> for AuthError {
    fn from(value: eyre::Report) -> Self {
        AuthError::EyreReport(value)
    }
}

impl From<VerifyError> for AuthError {
    fn from(value: VerifyError) -> Self {
        AuthError::VerifyError(value)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AuthError::SurrealError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AuthError::EyreReport(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AuthError::VerifyError(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            AuthError::NoUser => (StatusCode::UNAUTHORIZED, String::from("No user found with that username"))
        };

        (status, body).into_response()
    }
}

#[debug_handler]
pub async fn signup_handler(
    State(state): State<Arc<AppState>>,
    Json(json): Json<SignupReq>,
) -> Result<StatusCode, AuthError> {
    state.db.use_ns("dev").use_db("pokewarp").await?;



    let password_hash = generate_hash(json.password);

    let created: UserRecord = state.db.create("user").content(UserRecord {
        name: json.name,
        password_hash,
    }).await?;

    Ok(StatusCode::OK.into())
}

type Auth = AuthContext<Thing, User, SurrealUserStore, ()>;

#[debug_handler]
pub async fn login_handler(State(state): State<Arc<AppState>>, mut auth: Auth, Json(json): Json<LoginReq>) -> Result<StatusCode, AuthError> {
    
    state.db.use_ns("dev").use_db("pokewarp").await?;

    let mut query: Response = state.db.query("SELECT * FROM user WHERE name = $name LIMIT 1;").bind(("name", json.name)).await?;
    let user: Option<User> = query.take(0)?;

    if let Some(user) = user {

        match verify_password(json.password, std::str::from_utf8(user.get_password_hash().expose_secret()).unwrap()) {
            Ok(_) => {
                auth.login(&user).await?;
            },
            Err(err) => return Err(AuthError::from(err))
        }

    } else {
        return Err(AuthError::NoUser)
    }

    Ok(StatusCode::OK.into())
}