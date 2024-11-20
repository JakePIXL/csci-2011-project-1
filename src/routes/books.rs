use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use tracing::warn;

use crate::views::books::{Book, NewBook, SearchParams, UpdateBook};

pub fn books_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .service(get_books)
            .service(create_book)
            .service(update_book)
            .service(get_book_by_id)
            .service(delete_by_id),
    );
}

#[get("/")]
pub async fn get_books(
    query: web::Query<SearchParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let order_by = match query.get_order_by().as_str() {
        "id" | "title" | "author" | "category" | "status" | "created_at" => query.get_order_by(),
        _ => "id".to_string(),
    };

    let status = query.get_status();
    let status_value = status.as_str();

    let q = format!(
        r#"
        SELECT
            id,
            title,
            author,
            category,
            status
        FROM BOOKS
        WHERE
            (? IS NULL OR title LIKE CONCAT('%', ?, '%'))
            AND (? IS NULL OR author LIKE CONCAT('%', ?, '%'))
            AND (? IS NULL OR category = ?)
            AND (CASE
                WHEN ? IS NULL THEN TRUE
                WHEN ? = 'all' THEN TRUE
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
            {}
        LIMIT ? OFFSET ?
        "#,
        query.get_order().as_str()
    );

    let results: Vec<Book> = match sqlx::query_as(&q)
        .bind(query.title.clone())
        .bind(query.title.clone().unwrap_or("".to_string()))
        .bind(query.author.clone())
        .bind(query.author.clone().unwrap_or("".to_string()))
        .bind(query.category.clone())
        .bind(query.category.clone().unwrap_or("".to_string()))
        .bind(status_value)
        .bind(status_value)
        .bind(status_value)
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(order_by.as_str())
        .bind(query.limit.unwrap_or(10))
        .bind(query.get_offset())
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(books) => books,
        Err(error) => {
            warn!("Failed to fetch books: {:?}", error);
            return HttpResponse::InternalServerError().finish();
        }
    };

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
    data: web::Json<UpdateBook>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let book = match sqlx::query_as!(
        Book,
        r#"
            SELECT id, title, author, category, status
            FROM BOOKS
            WHERE id = ?
        "#,
        id.clone()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(book) => book,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let result = sqlx::query!(
        r#"
            UPDATE BOOKS
            SET title = ?, author = ?, category = ?, status = ?
            WHERE id = ?
        "#,
        if data.0.title.is_none() {
            book.title
        } else {
            data.0.title.unwrap()
        },
        if data.0.author.is_none() {
            book.author
        } else {
            data.0.author.unwrap()
        },
        if data.0.category.is_none() {
            book.category
        } else {
            data.0.category
        },
        if data.0.status.is_none() {
            None
        } else {
            Some(data.0.status.unwrap().to_string())
        },
        id.clone()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => {
            warn!("Failed to update book: {:?}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
async fn get_book_by_id(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Book,
        r#"
            SELECT id, title, author, category, status
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

#[delete("/{id}")]
async fn delete_by_id(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"
            DELETE FROM BOOKS
            WHERE id = ?
        "#,
        id.clone()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
