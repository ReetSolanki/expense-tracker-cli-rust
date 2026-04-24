pub mod models;
pub mod storage;
pub mod commands;

use clap::{Parser, Subcommand};
use models::Expense;
// use serde_json::{self, to_string_pretty};

use crate::{commands::{add, delete, list}, storage::{load_expenses, save_expenses}};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Delete { id: i32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Delete { id } => { 
            delete(id);
        }
    }
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