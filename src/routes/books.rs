use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::views::books::{Book, NewBook, SearchParams};

pub fn books_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .service(get_books)
            .service(create_book)
            .service(update_book)
            .service(get_by_id),
    );
}

#[get("/")]
pub async fn get_books(
    query: web::Query<SearchParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let order_by = match query.get_order_by().as_str() {
        "id" | "title" | "author" | "category" | "status" | "created_at" => query.get_order_by(),
        _ => "id".to_string(), // Default to id if invalid column specified
    };

    let status = query.get_status();
    let status_value = status.as_str();

    // This took me a while to figure out, but the SQL query below is a bit complex.
    // The query is using a CASE statement to check if the status is NULL. If it is, it will return TRUE.
    // it also allows for querying by each column and ordering by each column.
    let results = sqlx::query_as!(
        Book,
        r#"
            SELECT *
            FROM BOOKS
            WHERE
                (? IS NULL OR title LIKE CONCAT('%', ?, '%'))
                AND (? IS NULL OR author LIKE CONCAT('%', ?, '%'))
                AND (? IS NULL OR category = ?)
                AND (CASE
                    WHEN ? IS NULL THEN TRUE
                    ELSE status = ?
                    END)
            ORDER BY
                CASE
                    WHEN ? = 'id' THEN id
                    WHEN ? = 'title' THEN title
                    WHEN ? = 'author' THEN author
                    WHEN ? = 'category' THEN category
                    WHEN ? = 'status' THEN status
                    ELSE id
                END
            LIMIT ? OFFSET ?
            "#,
        query.title.clone(),
        query.title.clone().unwrap_or("".to_string()),
        query.author.clone(),
        query.author.clone().unwrap_or("".to_string()),
        query.category.clone(),
        query.category.clone().unwrap_or("".to_string()),
        status_value,
        status_value,
        order_by.as_str(),
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

#[post("/")]
async fn create_book(data: web::Json<NewBook>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"
            INSERT INTO BOOKS (title, author, category)
            VALUES (?, ?, ?)
        "#,
        data.title,
        data.author,
        data.category
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
async fn update_book(
    id: web::Path<i32>,
    data: web::Json<NewBook>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let book = sqlx::query_as!(
        Book,
        r#"
            SELECT *
            FROM BOOKS
            WHERE id = ?
        "#,
        id.clone()
    )
    .fetch_one(pool.get_ref())
    .await;

    match book {
        Ok(_) => {
            let result = sqlx::query!(
                r#"
                    UPDATE BOOKS
                    SET title = ?, author = ?, category = ?
                    WHERE id = ?
                "#,
                data.title,
                data.author,
                data.category,
                id.clone()
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}")]
async fn get_by_id(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Book,
        r#"
            SELECT *
            FROM BOOKS
            WHERE id = ?
        "#,
        id.clone()
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(book) => HttpResponse::Ok().json(json!(book)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
