use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde_json::json;

pub fn members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/members"));
}
