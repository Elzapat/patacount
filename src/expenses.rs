use crate::{model::Group, DB_NAME};
use actix_web::{get, post, put, web, App, HttpResponse};
use mongodb::{bson::Uuid, Client, Collection};

pub fn expenses_config() {}
