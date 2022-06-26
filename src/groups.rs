use crate::{model::Group, DB_NAME};
use actix_web::{get, post, put, web, App, HttpResponse};
use mongodb::{bson::Uuid, Client, Collection};

const GROUPS_COLLECTION: &str = "groups";

pub fn groups_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            .route("/new", web::get().to(create_group))
            .route("/{group_uuid}/add_user", web::post().to(add_user_to_group)),
    );
}

async fn create_group(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLLECTION);
    HttpResponse::Ok().body("test")
}

async fn add_user_to_group(client: web::Data<Client>, group_uuid: web::Path<Uuid>) -> HttpResponse {
    unimplemented!()
}
