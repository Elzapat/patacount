mod groups;
pub mod model;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::Uuid, Client, Collection};

pub const DB_NAME: &str = "patacount";
const USERS_COLLECTION: &str = "users";
const EXPENSES_COLLECTION: &str = "expenses";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to {uri}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(web::scope("/api/v1").configure(groups::groups_config))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
