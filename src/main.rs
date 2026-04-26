pub mod models;
pub mod storage;
pub mod commands;

use clap::{Command, arg, command, ArgGroup};
use models::Expense;
// use serde_json::{self, to_string_pretty};

use crate::{commands::{add, delete, list}, storage::{load_expenses, save_expenses}};


fn main() {
    let matches = command!()
        .arg(arg!(--two <VALUE>))
        .arg(arg!(--one <VALUE>))
        .group(
            ArgGroup::new("action")
                .args(["one", "two"])
                .required(true),
        )
        .get_matches();
    
}

// let expenses_vector = load_expenses();

    // let test_expense = Expense{
    //     id: expenses_vector.last().map_or(1, |e| e.id + 1),
    //     amount: 34.0,
    //     category: "food".to_string(),
    //     note: "Just a test".to_string(),
    //     date: "2025-04-23".to_string()  
    // };

    
    // add(test_expense);
    // delete(5); 
    // list(Some("food".to_string()));