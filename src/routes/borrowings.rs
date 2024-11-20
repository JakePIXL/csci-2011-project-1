use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;

use crate::views::{
    books::Book,
    borrowings::{BorrowParams, BorrowRequest, BorrowedBook},
    members::Member,
};

pub fn borrowings_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/borrows")
            .service(new_borrowing)
            .service(return_books)
            .service(get_borrows_by_id)
            .service(get_all_borrowings),
    );
}

#[post("/{id}")]
async fn new_borrowing(
    id: web::Path<i32>,
    data: web::Json<BorrowRequest>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let id = id.into_inner();

    // Check if member exists
    let member = match sqlx::query_as!(
        Member,
        r#"SELECT id, first_name, last_name, email, phone FROM MEMBERS WHERE id = ?"#,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(member) => member,
        Err(_) => return HttpResponse::NotFound().json("Member not found"),
    };

    // Check if book exists and is available
    match sqlx::query!(
        r#"SELECT id, title, author, category, status FROM BOOKS WHERE id = ? AND status = 'available'"#,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(_) => {},
        Err(_) => return HttpResponse::NotFound().json("Book not found or not available"),
    };

    // Check if book is already borrowed by the member (just in case)
    let existing_borrow = sqlx::query!(
        r#"SELECT id FROM BORROWINGS
           WHERE member_id = ? AND book_id = ? AND return_date IS NULL"#,
        member.id,
        data.0.book_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    if let Ok(Some(_)) = existing_borrow {
        return HttpResponse::BadRequest().json("Book is already borrowed by this member");
    }

    // Create a new borrowing record
    match sqlx::query!(
        r#"INSERT INTO BORROWINGS (member_id, book_id, borrow_date)
           VALUES (?, ?, CURRENT_TIMESTAMP)"#,
        member.id,
        data.0.book_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    // Update book availability
    match sqlx::query!(
        r#"UPDATE BOOKS SET status = 'borrowed' WHERE id = ?"#,
        data.0.book_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Book borrowed successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_borrows_by_id(
    id: web::Path<i32>,
    params: web::Query<BorrowParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let id = id.into_inner();

    let status = params.get_status();
    let status_value = status.as_str();

    // Get all borrowed books by the member
    // If current is true, get only the books that are not returned yet
    // If current is false, gets all the books that are or have been borrowed
    // orders the results by borrow date but allows the user to change the order
    let q = format!(
        r#"
        SELECT id, title, author, borrower, borrower_id, borrow_date, return_date, status FROM BORROWED_BOOKS
        WHERE borrower_id = ?
        AND (CASE
            WHEN ? IS NULL THEN TRUE
            WHEN ? = 'all' THEN TRUE
            ELSE status = ?
            END)
        ORDER BY borrow_date {}
    "#,
        params.0.get_order().as_str()
    );

    let results: Vec<BorrowedBook> = match sqlx::query_as(&q)
        .bind(id)
        .bind(status_value)
        .bind(status_value)
        .bind(status_value)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(results) => results,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(json!(results))
}

#[post("/return/")]
async fn return_books(
    data: web::Json<BorrowRequest>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let book = match sqlx::query_as!(
        Book,
        r#"
            SELECT id, title, author, category, status
            FROM BOOKS
            WHERE id = ?
        "#,
        data.book_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(book) => book,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    tracing::info!("Book: {:?}", json!(book));

    if book.status.as_str() != "borrowed" {
        return HttpResponse::BadRequest().json(json!("Book is not borrowed"));
    }

    let borrowed = match sqlx::query!(
        r#"
        SELECT id, book_id, member_id, borrow_date, return_date FROM BORROWINGS WHERE book_id = ? AND return_date IS NULL
        "#,
        book.id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(borrowed) => borrowed,
        Err(_) => return HttpResponse::NotFound().json(json!("Book is not borrowed")),
    };

    match sqlx::query!(
        r#"
            UPDATE BOOKS
            SET status = 'available'
            WHERE id = ?
        "#,
        book.id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    match sqlx::query!(
        r#"
            UPDATE BORROWINGS
            SET return_date = ?
            WHERE id = ?
        "#,
        Utc::now(),
        borrowed.id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!("Books returned")),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/")]
async fn get_all_borrowings(
    params: web::Query<BorrowParams>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    let status = params.get_status();
    let status_value = status.as_borrowing_str();

    let q = format!(
        r#"
        SELECT id, title, author, borrower, borrower_id, borrow_date, return_date, status
        FROM BORROWED_BOOKS
        WHERE (CASE
            WHEN ? = 'all' THEN TRUE
            ELSE status = ?
            END)
        ORDER BY borrow_date {}
        "#,
        params.get_order()
    );

    let results: Vec<BorrowedBook> = match sqlx::query_as(&q)
        .bind(status_value)
        .bind(status_value)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(results) => results,
        Err(e) => {
            tracing::warn!("Error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(json!(results))
}

#[delete("/{id}")]
async fn delete_borrowing(id: web::Path<i32>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let id = id.into_inner();

    // grab the borrowed book entry
    let borrowing = match sqlx::query!(
        r#"
        SELECT id, book_id, member_id, borrow_date, return_date
        FROM BORROWINGS
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(borrowing) => borrowing,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    // delete the borrowed book entry
    match sqlx::query!(
        r#"
        DELETE FROM BORROWINGS
        WHERE id = ?
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    // set the original book status to available (just in case)
    match sqlx::query!(
        r#"
        UPDATE BOOKS
        SET status = 'available'
        WHERE id = ?
        "#,
        borrowing.book_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!("Borrowing deleted")),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
