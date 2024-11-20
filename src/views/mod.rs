use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;

use sqlx::{
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
};

pub mod books;
pub mod borrowings;
pub mod members;

#[derive(Serialize, Deserialize, Clone)]
pub enum Order {
    #[serde(rename = "asc", alias = "ASC", alias = "Asc")]
    ASC,
    #[serde(rename = "desc", alias = "DESC", alias = "Desc")]
    DESC,
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::ASC => "ASC",
            Order::DESC => "DESC",
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::ASC),
            "desc" => Ok(Order::DESC),
            _ => Err(format!("Unknown order: {}", s)),
        }
    }
}

impl From<Option<String>> for Order {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) => s.parse().unwrap_or(Order::ASC),
            None => Order::DESC,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    #[serde(rename = "available", alias = "AVAILABLE", alias = "Available")]
    Available,
    #[serde(rename = "borrowed", alias = "BORROWED", alias = "Borrowed")]
    Borrowed,
    #[serde(rename = "all", alias = "ALL", alias = "All")]
    All,
}

impl Status {
    pub fn as_null_str(&self) -> Option<&str> {
        match self {
            Status::Available => Some("available"),
            Status::Borrowed => Some("borrowed"),
            Status::All => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Status::Available => "available",
            Status::Borrowed => "borrowed",
            Status::All => "all",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available" => Ok(Status::Available),
            "borrowed" => Ok(Status::Borrowed),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

impl From<Option<String>> for Status {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) => s.parse().unwrap_or(Status::Available),
            None => Status::Available,
        }
    }
}

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::Available => "available".to_string(),
            Status::Borrowed => "borrowed".to_string(),
            Status::All => "all".to_string(),
        }
    }
}

impl sqlx::encode::Encode<'_, sqlx::MySql> for Status {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let s = self.as_str();
        buf.extend_from_slice(s.as_bytes());
        Ok(IsNull::No)
    }
}

impl sqlx::decode::Decode<'_, sqlx::MySql> for Status {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as sqlx::decode::Decode<sqlx::MySql>>::decode(value)?;
        Ok(s.parse().unwrap_or(Status::Available))
    }
}

impl sqlx::Type<sqlx::MySql> for Status {
    fn type_info() -> MySqlTypeInfo {
        <String as sqlx::Type<sqlx::MySql>>::type_info()
    }
}
