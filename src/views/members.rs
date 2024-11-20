use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::Order;

// Member structs
#[derive(Serialize, Deserialize, FromRow)]
pub struct Member {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMember {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMember {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberSearchParams {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
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
        const ALLOWED_COLUMNS: [&str; 5] = ["id", "first_name", "last_name", "email", "phone"];

        match &self.order_by {
            Some(column) if ALLOWED_COLUMNS.contains(&column.as_str()) => column.clone(),
            _ => "id".to_string(),
        }
    }

    pub fn get_order(&self) -> Order {
        self.order.clone().unwrap_or(Order::ASC)
    }
}
