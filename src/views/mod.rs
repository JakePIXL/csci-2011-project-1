use serde::{Deserialize, Serialize};

pub mod books;
pub mod members;

#[derive(Serialize, Deserialize, Clone)]
pub enum Order {
    Asc,
    Desc,
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
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
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err(format!("Unknown order: {}", s)),
        }
    }
}

impl From<Option<String>> for Order {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) => s.parse().unwrap_or(Order::Asc),
            None => Order::Asc,
        }
    }
}
