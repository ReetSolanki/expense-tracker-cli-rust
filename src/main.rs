pub mod models;
pub mod storage;
pub mod commands;

use clap::{Arg, Command, arg, command, value_parser};
use models::Expense;

use crate::{commands::{add, delete, list}};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("delete")
                .arg(arg!(<delete>)
                .value_parser(value_parser!(i32)))
        )
        .subcommand(
            Command::new("list")
                .arg(arg!([list]))   // square brackets to make the value optional (list travel / list) 
        )
        .subcommand(
            Command::new("add")
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .long("amount")//Adding long, makes the arguments named than being a positional argument
                        .value_parser(value_parser!(f64))
                )
                .arg(
                    Arg::new("category")
                        .required(true)
                        .long("category")
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("note")
                        .required(true)
                        .long("note")
                        .value_parser(value_parser!(String))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("delete", sub_m)) => {
            let val: i32 = *sub_m.get_one::<i32>("delete").unwrap();
            delete(val);
        }
        Some(("list", sub_m)) => {
            let val = sub_m.get_one::<String>("list");
            list(val.cloned());
        }
        Some(("add", sub_m)) => {
            let expense = Expense{
                id: 0,
                amount: *sub_m.get_one::<f64>("amount").unwrap(),
                category: sub_m.get_one::<String>("category").unwrap().clone(),
                note: sub_m.get_one::<String>("note").unwrap().clone(),
                date: "2025-04-23".to_string()  ,
            };
            add(expense);
        }
        _ => {println!("Pleaes provide a valid command. Use --help for usage.")}
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