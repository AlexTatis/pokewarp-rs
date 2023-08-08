use axum::{
    routing::{get, post},
    Router,
};
use axum_login::{AuthLayer, axum_sessions::SessionLayer, axum_sessions::async_session::MemoryStore};
use handlers::auth::SurrealUserStore;
use rand::Rng;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root};
use std::{net::SocketAddr, sync::Arc};

pub mod handlers;
pub mod middleware;

pub struct AppState {
    pub db: Arc<Surreal<Client>>
}

#[tokio::main]
async fn main() {

    let db = Arc::new(Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Error when connecting to DB!"));
    let mut secret: [u8; 64] = [0u8; 64];
    rand::thread_rng().fill(&mut secret[..]);
    
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("Could not login into the DB!");

    let session_store = MemoryStore::new();
    let user_store = SurrealUserStore::new(db.clone());

    let session_layer = SessionLayer::new(session_store, &secret);
    let auth_layer = AuthLayer::new(user_store, &secret);

    
    // type RequireAuth = RequireAuthorizationLayer<Thing, User, ()>;


    let shared_state = Arc::new(AppState {
        db: db.clone()
    });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/parse", post(handlers::parse_sav::parse_handler))
        .route("/signup", post(handlers::auth::signup_handler))
        .route("/login", post(handlers::auth::login_handler))
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(shared_state);
        

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!!"
}
