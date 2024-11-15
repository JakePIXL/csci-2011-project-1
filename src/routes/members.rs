use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::views::members::{Member, MemberSearchParams};

pub fn members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/members").service(get_members));
}

#[get("/")]
pub async fn get_members(
    query: web::Query<MemberSearchParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let order_by = match query.get_order_by().as_str() {
        "id" | "name" | "email" | "phone" | "status" | "created_at" => query.get_order_by(),
        _ => "id".to_string(),
    };

    let results = sqlx::query_as!(
        Member,
        r#"
            SELECT *
            FROM MEMBERS
            WHERE
                (? IS NULL OR name LIKE CONCAT('%', ?, '%'))
                AND (? IS NULL OR email LIKE CONCAT('%', ?, '%'))
                AND (? IS NULL OR phone LIKE CONCAT('%', ?, '%'))
            ORDER BY
                CASE
                    WHEN ? = 'id' THEN id
                    WHEN ? = 'name' THEN name
                    WHEN ? = 'email' THEN email
                    WHEN ? = 'phone' THEN phone
                    ELSE id
                END
            LIMIT ? OFFSET ?
            "#,
        query.name.clone(),
        query.name.clone().unwrap_or("".to_string()),
        query.email.clone(),
        query.email.clone().unwrap_or("".to_string()),
        query.phone.clone(),
        query.phone.clone().unwrap_or("".to_string()),
        order_by.as_str(),
        order_by.as_str(),
        order_by.as_str(),
        order_by.as_str(),
        query.limit.unwrap_or(10),
        query.get_offset()
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(json!(results))
}
