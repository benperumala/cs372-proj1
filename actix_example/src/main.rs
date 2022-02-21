mod db;  // Tell Rust we want to use the db.rs module

// File imports
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use db::parse_db;

// Say we want to use the external Serialize Deserialize (serde) crate
// #[macro_use] says we want to use it's macros/traits
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

/// A route which returns a JSON object containing all the songs within the database
#[get("/songs")]
async fn all_songs() -> Result<impl Responder> {
    println!("GET /songs");
    let d = parse_db();
    // println!("Database: {:?}", d);
    Ok(web::Json(d))
}

/// Get a specific song from the database (id=index from /songs route)
#[get("/songs/{id}")]
async fn get_song(id: web::Path<usize>) -> Result<impl Responder> {
    println!("GET /songs/{}", id);
    let d = parse_db();
    let s = d.get_song(*id);
    Ok(web::Json(s.clone()))
}

/// Echo back messages that are sent to this endpoint
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("POST /echo (data={})", req_body);
    HttpResponse::Ok().body(req_body)
}

/// Display `src/welcome.html` which is an HTML file
/// a user's browser can view. It contains a table 
#[get("/")]
async fn homepage(_req_body: String) -> impl Responder {
    println!("GET /");
    HttpResponse::Ok()
        .body(include_str!("static/welcome.html"))
}

/// Define the main web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1:8080";
    println!("Starting HTTP server on {}", host);
    HttpServer::new(|| {
        App::new()
            .service(homepage)
            .service(all_songs)
            .service(get_song)
            .service(echo)
    })
    .bind(host)?
    .run()
    .await
}
