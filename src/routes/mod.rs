pub mod books;
pub mod members;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/health_check")]
pub async fn health_check(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
