use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub status: BookStatus,
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
    pub status: Option<BookStatus>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BookStatus {
    Available,
    Borrowed,
    All,
}

impl BookStatus {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            BookStatus::Available => Some("available"),
            BookStatus::Borrowed => Some("borrowed"),
            BookStatus::All => None,
        }
    }
}

impl std::str::FromStr for BookStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(BookStatus::Available),
            "borrowed" => Ok(BookStatus::Borrowed),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

impl From<Option<String>> for BookStatus {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) => s.parse().unwrap_or(BookStatus::Available),
            None => BookStatus::Available,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BookOrder {
    Asc,
    Desc,
}

impl BookOrder {
    pub fn as_str(&self) -> &str {
        match self {
            BookOrder::Asc => "ASC",
            BookOrder::Desc => "DESC",
        }
    }
}

impl fmt::Display for BookOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for BookOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(BookOrder::Asc),
            "desc" => Ok(BookOrder::Desc),
            _ => Err(format!("Unknown order: {}", s)),
        }
    }
}

impl From<Option<String>> for BookOrder {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) => s.parse().unwrap_or(BookOrder::Asc),
            None => BookOrder::Asc,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub status: Option<BookStatus>,
    pub order: Option<BookOrder>,
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

    pub fn get_order(&self) -> BookOrder {
        if self.order.is_none() {
            BookOrder::Asc
        } else {
            self.order.clone().unwrap()
        }
    }

    pub fn get_status(&self) -> BookStatus {
        self.status.clone().unwrap_or(BookStatus::All)
    }
}
