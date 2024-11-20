use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::{Order, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub status: Option<Status>,
    pub order: Option<Order>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
}

impl SearchParams {
    pub fn get_offset(&self) -> u32 {
        if self.page.is_none() || self.page.unwrap() == 0 {
            0
        } else {
            (self.page.unwrap() - 1) * self.limit.unwrap_or(10)
        }
    }

    pub fn get_order_by(&self) -> String {
        const ALLOWED_COLUMNS: [&str; 5] = ["id", "title", "author", "category", "status"];

        match &self.order_by {
            Some(column) if ALLOWED_COLUMNS.contains(&column.as_str()) => column.clone(),
            _ => "id".to_string(),
        }
    }

    pub fn get_order(&self) -> Order {
        if self.order.is_none() {
            Order::ASC
        } else {
            self.order.clone().unwrap()
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone().unwrap_or(Status::All)
    }
}
