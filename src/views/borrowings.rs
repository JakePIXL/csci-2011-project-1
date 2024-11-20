use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::{Order, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Borrowing {
    pub id: i32,
    pub book_id: Option<i32>,
    pub member_id: Option<i32>,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct NewBorrowing {
    pub book_id: i32,
    pub member_id: i32,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBorrowing {
    pub return_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct BorrowingSearchParams {
    pub book_id: Option<i32>,
    pub member_id: Option<i32>,
    pub status: Option<Status>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub order: Option<Order>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

impl BorrowingSearchParams {
    pub fn get_offset(&self) -> u32 {
        if self.page.is_none() || self.page.unwrap() == 0 {
            0
        } else {
            (self.page.unwrap() - 1) * self.limit.unwrap_or(10)
        }
    }

    pub fn get_order_by(&self) -> String {
        const ALLOWED_COLUMNS: [&str; 5] =
            ["id", "book_id", "member_id", "borrow_date", "return_date"];

        match &self.order_by {
            Some(column) if ALLOWED_COLUMNS.contains(&column.as_str()) => column.clone(),
            _ => "id".to_string(),
        }
    }

    pub fn get_order(&self) -> Order {
        self.order.clone().unwrap_or(Order::ASC)
    }
}

#[derive(Deserialize)]
pub struct BorrowRequest {
    pub book_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BorrowedBook {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub borrower: String,
    pub borrower_id: i32,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct BorrowParams {
    pub current: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub order: Option<Order>,
    pub status: Option<Status>,
}

impl BorrowParams {
    pub fn get_offset(&self) -> u32 {
        if self.page.is_none() || self.page.unwrap() == 0 {
            0
        } else {
            (self.page.unwrap() - 1) * self.limit.unwrap_or(10)
        }
    }

    pub fn get_order(&self) -> Order {
        self.order.clone().unwrap_or(Order::ASC)
    }

    pub fn get_status(&self) -> Status {
        self.status.clone().unwrap_or(Status::All)
    }
}
