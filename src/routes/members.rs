use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::views::members::{Member, MemberSearchParams, NewMember, UpdateMember};

pub fn members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            .service(get_members)
            .service(create_member)
            .service(update_member)
            .service(get_by_id)
            .service(delete_by_id),
    );
}

#[get("/")]
pub async fn get_members(
    query: web::Query<MemberSearchParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let order_by = match query.get_order_by().as_str() {
        "id" | "first_name" | "last_name" | "email" | "phone" | "status" | "created_at" => {
            query.get_order_by()
        }
        _ => "id".to_string(),
    };

    // This query took a bulk of my time to create as I had to look up CASE statements in SQL and MariaDB
    // and figure out how to use them in Rust. I also had to figure out how to use the url parameters
    // as search terms in each column, along with the order_by parameter to sort the results without wasting
    // processing power.
    let q = format!(
        r#"
        SELECT id, first_name, last_name, email, phone
        FROM MEMBERS
        WHERE
            (? IS NULL OR first_name LIKE CONCAT('%', ?, '%'))
            AND (? IS NULL OR last_name LIKE CONCAT('%', ?, '%'))
            AND (? IS NULL OR email LIKE CONCAT('%', ?, '%'))
            AND (? IS NULL OR phone LIKE CONCAT('%', ?, '%'))
        ORDER BY
            CASE
                WHEN ? = 'id' THEN id
                WHEN ? = 'first_name' THEN first_name
                WHEN ? = 'last_name' THEN last_name
                WHEN ? = 'email' THEN email
                WHEN ? = 'phone' THEN phone
                ELSE id
            END
            {}
        LIMIT ? OFFSET ?
        "#,
        query.get_order().as_str()
    );

    let results: Vec<Member> = sqlx::query_as(&q)
        .bind(query.first_name.clone())
        .bind(query.first_name.clone().unwrap_or("".to_string()))
        .bind(query.last_name.clone())
        .bind(query.last_name.clone().unwrap_or("".to_string()))
        .bind(query.email.clone())
        .bind(query.email.clone().unwrap_or("".to_string()))
        .bind(query.phone.clone())
        .bind(query.phone.clone().unwrap_or("".to_string()))
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(query.limit.unwrap_or(10))
        .bind(query.get_offset())
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(json!(results))
}

#[post("/")]
async fn create_member(
    data: web::Json<NewMember>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
            INSERT INTO MEMBERS (first_name, last_name, email, phone)
            VALUES (?, ?, ?, ?)
        "#,
        data.first_name,
        data.last_name,
        data.email,
        data.phone
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
async fn update_member(
    id: web::Path<i32>,
    data: web::Json<UpdateMember>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    // Check if the member exists
    let member = match sqlx::query_as!(
        Member,
        r#"
            SELECT id, first_name, last_name, email, phone
            FROM MEMBERS
            WHERE id = ?
        "#,
        id.clone()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(member) => member,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    // Update the member with the new data
    let result = sqlx::query!(
        r#"
            UPDATE MEMBERS
            SET first_name = ?, last_name = ?, email = ?, phone = ?
            WHERE id = ?
        "#,
        data.first_name.clone().unwrap_or(member.first_name),
        data.last_name.clone().unwrap_or(member.last_name),
        data.email.clone().unwrap_or(member.email),
        if data.phone.clone().is_some() {
            data.phone.clone()
        } else {
            member.phone
        },
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_by_id(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    match sqlx::query_as!(
        Member,
        r#"
            SELECT id, first_name, last_name, email, phone
            FROM MEMBERS
            WHERE id = ?
        "#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(member) => HttpResponse::Ok().json(json!(member)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/{id}")]
async fn delete_by_id(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"
            DELETE FROM MEMBERS
            WHERE id = ?
        "#,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
