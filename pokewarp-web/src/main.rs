use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use pokeparser::{pokemon::pk5::PK5, pokedex, saves};
use std::{net::SocketAddr, ops::Deref};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/parse", post(parse_sav));

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


async fn parse_sav(mut multipart: Multipart) -> (StatusCode, Json<[PK5; 6]>) {
    let pokemons = pokedex::Pokemons::new("./data/pokedex.json");
    let abilities = pokedex::Abilities::new("./data/abilities.json");
    let moves = pokedex::Moves::new("./data/moves.json");
    let natures = pokedex::Natures::new("./data/natures.json");
    let items = pokedex::Items::new("./data/items.json");


    if let Some(field) = multipart.next_field().await.unwrap() {

        let field_text = field.bytes().await.unwrap();
        let gen5_save = saves::gen5::Save::new(field_text.deref(), &pokemons, &abilities, &moves, &natures, &items);
        let pkms = gen5_save.get_party();
        return (StatusCode::OK, Json(pkms))
    }

    (StatusCode::OK, Json([PK5::empty(), PK5::empty(), PK5::empty(), PK5::empty(), PK5::empty(), PK5::empty()]))


}
