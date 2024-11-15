use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::Order;

// Member structs
#[derive(Serialize, Deserialize, FromRow)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMember {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMember {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

// Borrowing structs
#[derive(Serialize, Deserialize, FromRow)]
pub struct Borrowing {
    pub id: i32,
    pub book_id: i32,
    pub member_id: i32,
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

#[derive(Serialize, Deserialize, Clone)]
pub enum BorrowingStatus {
    Active,
    Returned,
    All,
}

impl BorrowingStatus {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            BorrowingStatus::Active => Some("active"),
            BorrowingStatus::Returned => Some("returned"),
            BorrowingStatus::All => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BorrowingSearchParams {
    pub book_id: Option<i32>,
    pub member_id: Option<i32>,
    pub status: Option<BorrowingStatus>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub order: Option<Order>, // Reusing BookOrder from your example
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
        self.order.clone().unwrap_or(Order::Asc)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemberSearchParams {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub order: Option<Order>, // Reusing BookOrder from your example
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

impl MemberSearchParams {
    pub fn get_offset(&self) -> u32 {
        if self.page.is_none() || self.page.unwrap() == 0 {
            0
        } else {
            (self.page.unwrap() - 1) * self.limit.unwrap_or(10)
        }
    }

    pub fn get_order_by(&self) -> String {
        const ALLOWED_COLUMNS: [&str; 4] = ["id", "name", "email", "phone"];

        match &self.order_by {
            Some(column) if ALLOWED_COLUMNS.contains(&column.as_str()) => column.clone(),
            _ => "id".to_string(),
        }
    }

    pub fn get_order(&self) -> Order {
        self.order.clone().unwrap_or(Order::Asc)
    }
}
