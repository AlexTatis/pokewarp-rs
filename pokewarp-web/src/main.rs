use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Result},
    routing::{get, post},
    Json, Router,
};
use pokeparser::{
    pokedex,
    saves::{self, gen5::Party},
};
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

enum ParseFileError {
    ParseError,
    NoFileUploaded,
    IncorrectSize
}

impl IntoResponse for ParseFileError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            ParseFileError::ParseError => "Error while parsing save file",
            ParseFileError::NoFileUploaded => "No file was uploaded",
            ParseFileError::IncorrectSize => "File size is different from 512 kB"
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

async fn parse_sav(mut multipart: Multipart) -> Result<Json<Party>> {
    
    if let Some(field) = multipart.next_field().await.unwrap() {
        let field_text = field.bytes().await.unwrap();
        
        if field_text.len() != 512 * 1024 { return Err(ParseFileError::IncorrectSize.into()) }

        let pokemons = pokedex::Pokemons::new("./data/pokedex.json");
        let abilities = pokedex::Abilities::new("./data/abilities.json");
        let moves = pokedex::Moves::new("./data/moves.json");
        let natures = pokedex::Natures::new("./data/natures.json");
        let items = pokedex::Items::new("./data/items.json");
        
        
        let gen5_save = saves::gen5::Save::new(
            field_text.deref(),
            &pokemons,
            &abilities,
            &moves,
            &natures,
            &items,
        );

        match gen5_save.get_party() {
            Some(pkms) => return Ok(Json(pkms)),
            None => return Err(ParseFileError::ParseError.into()),
        }
    }

    Err(ParseFileError::NoFileUploaded.into())
}
