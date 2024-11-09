use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::views::books::{Book, SearchParams};

pub fn books_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/books").service(get_books));
}

#[get("/")]
pub async fn get_books(
    _query: web::Query<SearchParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let results = sqlx::query_as!(
        Book,
        r#"
        SELECT *
        FROM BOOKS
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(json!(results))
}
