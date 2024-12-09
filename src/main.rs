
mod api;
mod types;
mod methods;
use api::{artifacts::get_artifact_substats, characters::{get_character, get_characters_by_search_params, get_characters}};
use rocket::{fs::FileServer, http::Method};
use rocket_cors::{ Cors, AllowedOrigins, CorsOptions, AllowedHeaders };

#[macro_use] extern crate rocket;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://0.0.0.0:3000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("address", "127.0.0.1")))
        .mount("/api", routes![
            get_characters,
            get_character,
            get_characters_by_search_params,
            get_artifact_substats
        ])
        .mount("/assets", FileServer::from("assets"))
        .attach(make_cors())
}