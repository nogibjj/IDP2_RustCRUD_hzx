#![allow(warnings)]
#![allow(clippy::all)]
use etl_rust::{extract, query, transform_load, create_general_item, delete_general_item, update_general_item};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the arugments. Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            extract(
                "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv",
                "data/GroceryDB_IgFPro.csv",
            ).expect("Error during extraction");
            println!("Extraction completed!");
        }
        "transform_load" => {
            transform_load("data/GroceryDB_IgFPro.csv")
                .expect("Error during transformation and loading");
            println!("Data loaded successfully!");
        }
        "query" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = query() {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("We only support sample function invocation. Usage: {} query [SQL query]", args[0]);
            }
        }
        // id = 1428 last
        "create" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = create_general_item(
                    "New Item",
                    42,
                    3.146,
                    2.71,
                    1.618,
                    0.123,
                    "Sample Name",
                    "Sample Node",
                ) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("We only support sample function invocation. Usage: {} query [SQL query]", args[0]);
            }
        }
        "delete" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = delete_general_item(1428) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("We only support sample function invocation. Usage: {} query [SQL query]", args[0]);
            }
        }
        "update" => {
            if args.len() <= 2 {
                // let query_str = &args[2];
                if let Err(err) = update_general_item(
                    1, // ID of the item to update
                    "Updated Item",
                    99,
                    4.56,
                    7.89,
                    5.4321,
                    0.987,
                    "Updated Name",
                    "Updated Node",
                ){
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("We only support sample function invocation. Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', 'create', 'delete', 'update' or 'query' command.");
        }
    }
}
