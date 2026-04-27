use serde_json::{from_str, to_string_pretty};

use crate::models::Expense;
use std::{fs, vec};

const FILE_PATH: &str = "./expenses.json";

// Read: From expenses.json and convert it to vector
pub fn load_expenses() -> Vec<Expense>{
    let data = match fs::read_to_string(FILE_PATH){
        Ok(content ) => content,
        Err(_) => return vec![],
    };

    return from_str::<Vec<Expense>>(&data)
            .expect("Failed to parse JSON file")
}

// Write: Save the Vec back to expenses.json
pub fn save_expenses(expenses: &Vec<Expense>){
    let data = to_string_pretty(expenses)
        .expect("Failed to serialize expense");
    
    fs::write(FILE_PATH, data)
        .expect("Failed to write to file");
}