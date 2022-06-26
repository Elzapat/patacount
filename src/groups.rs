use crate::{model::Group, DB_NAME};
use actix_web::{get, post, put, web, App, HttpResponse};
use mongodb::{bson::Uuid, Client, Collection};

const GROUPS_COLLECTION: &str = "groups";

pub fn groups_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            .service(new_group)
            .service(add_user_to_group),
    );
}

#[post("/new")]
async fn new_group(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLLECTION);
    HttpResponse::Ok().body("test")
}

#[post("/{group_uuid}/add_user")]
async fn add_user_to_group(client: web::Data<Client>, group_uuid: web::Path<Uuid>) -> HttpResponse {
    unimplemented!()
}
