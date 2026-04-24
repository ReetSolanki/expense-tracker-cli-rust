use serde::{Serialize, Deserialize};
// use serde_json::{to_string_pretty, from_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: i32,
    pub amount: f64,
    pub category: String,
    pub note: String,
    pub date: String  // "2025-04-23"
}