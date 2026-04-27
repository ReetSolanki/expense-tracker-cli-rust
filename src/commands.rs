use std::collections::HashMap;

// use clap::builder::Str;
use crate::models::Expense;
use crate::storage::*;

pub fn summary(){  // Show total spent per category
    let expenses_vector = load_expenses();
    let mut map: HashMap<String, f64> =  HashMap::new();
    
    expenses_vector.iter().for_each(|e| {
        *map.entry(e.category.clone()).or_insert(0.0) += e.amount;
    });

    println!("Category  Amount");
    for e in map{
        println!("{}:   {:.2}", e.0, e.1);
    }
}

pub fn add(mut new_expense: Expense){
    let mut expenses_vector = load_expenses();
    new_expense.id = expenses_vector.last().map_or(1, |e| e.id + 1);
    expenses_vector.push(new_expense);
    save_expenses(&expenses_vector);
}

pub fn list(category_type: Option<String>){
    let expenses_vector = load_expenses();
    
    let filtered_vector_iter: Vec<&Expense> = expenses_vector.iter()
        .filter(|e| match &category_type {
            Some(cat) => e.category == *cat,
            None => true
        })
        .collect();

    if filtered_vector_iter.is_empty(){
        println!("No expense found");
        return;
    }

    println!("{:#?}", filtered_vector_iter);
}

pub fn delete(id_to_delete: i32){
    let mut expenses_vector = load_expenses();

    match expenses_vector.iter().position(|e| e.id == id_to_delete) {
        Some(index) => {
            expenses_vector.remove(index);
            println!("Deleted expense with id {}", id_to_delete);
            save_expenses(&expenses_vector);
        }
        None => println!("No expense found with id {}", id_to_delete),
    }
}