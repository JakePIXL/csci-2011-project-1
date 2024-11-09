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
pub enum BookStatus {
    Available,
    Borrowed,
}

impl BookStatus {
    pub fn as_str(&self) -> &str {
        match self {
            BookStatus::Available => "available",
            BookStatus::Borrowed => "borrowed",
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

#[derive(Serialize, Deserialize)]
pub struct SearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub status: Option<BookStatus>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
