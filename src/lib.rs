#![allow(warnings)]
#![allow(clippy::all)]
use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;

use std::error::Error;

const DATABASE_FILE: &str = "GroceryDB.sqlite";

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut file = fs::File::create(file_path)?;
    std::io::copy(&mut response, &mut file).expect("Failed to copy content");
    println!("Extraction successful!");
    Ok(())
}

pub fn transform_load(dataset: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DATABASE_FILE)?;
    conn.execute("DROP TABLE IF EXISTS GroceryDB", [])?;
    conn.execute(
        "CREATE TABLE GroceryDB (
            id INTEGER PRIMARY KEY,
            general_name TEXT,
            count_products INTEGER,
            ingred_FPro REAL,
            avg_FPro_products REAL,
            avg_distance_root REAL,
            ingred_normalization_term REAL,
            semantic_tree_name TEXT,
            semantic_tree_node TEXT
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset)?;
    let mut stmt = conn.prepare(
        "INSERT INTO GroceryDB (
            general_name,
            count_products,
            ingred_FPro,
            avg_FPro_products,
            avg_distance_root,
            ingred_normalization_term,
            semantic_tree_name,
            semantic_tree_node
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    // As for why the & symbol is used, it relates to Rust's borrowing system. In the stmt.execute function, references are required to access the data within record. 
    //In Rust, passing a value to a function typically transfers ownership, and to avoid transferring ownership and allow multiple references to the same data, references are used.

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[1],
                    &record[2],
                    &record[3],
                    &record[4],
                    &record[5],
                    &record[6],
                    &record[7],
                    &record[8],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok(())
}

pub fn query() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DATABASE_FILE)?;
    if let Ok(mut stmt) = conn.prepare("SELECT * FROM GroceryDB") {
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, f64>(3)?,
                row.get::<usize, f64>(4)?,
                row.get::<usize, f64>(5)?,
                row.get::<usize, f64>(6)?,
                row.get::<usize, String>(7)?,
                row.get::<usize, String>(8)?,
            ))
        })?;

        println!("Top 5 rows of the GroceryDB table:");
        for result in results.take(5) {
            match result {
                Ok((
                    id,
                    general_name,
                    count_products,
                    ingred_FPro,
                    avg_FPro_products,
                    avg_distance_root,
                    ingred_normalization_term,
                    semantic_tree_name,
                    semantic_tree_node,
                )) => {
                    println!(
                        "Result: id={}, general_name={}, count_products={}, ingred_FPro={}, avg_FPro_products={}, avg_distance_root={}, ingred_normalization_term={}, semantic_tree_name={}, semantic_tree_node={}",
                        id,
                        general_name,
                        count_products,
                        ingred_FPro,
                        avg_FPro_products,
                        avg_distance_root,
                        ingred_normalization_term,
                        semantic_tree_name,
                        semantic_tree_node
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        eprintln!("Error preparing query.");
    }
    Ok(())
}

pub fn create_general_item(
    name: &str,
    count_products: i32,
    ingred_FPro: f64,
    avg_FPro_products: f64,
    avg_distance_root: f64,
    ingred_normalization_term: f64,
    semantic_tree_name: &str,
    semantic_tree_node: &str,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DATABASE_FILE)?;
    conn.execute(
        "INSERT INTO GroceryDB (
            general_name,
            count_products,
            ingred_FPro,
            avg_FPro_products,
            avg_distance_root,
            ingred_normalization_term,
            semantic_tree_name,
            semantic_tree_node
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            name,
            count_products,
            ingred_FPro,
            avg_FPro_products,
            avg_distance_root,
            ingred_normalization_term,
            semantic_tree_name,
            semantic_tree_node
        ],
    )?;
    Ok(())
}


pub fn update_general_item(
    id: i32,
    name: &str,
    count_products: i32,
    ingred_FPro: f64,
    avg_FPro_products: f64,
    avg_distance_root: f64,
    ingred_normalization_term: f64,
    semantic_tree_name: &str,
    semantic_tree_node: &str,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DATABASE_FILE)?;
    conn.execute(
        "UPDATE GroceryDB
        SET general_name = ?2,
            count_products = ?3,
            ingred_FPro = ?4,
            avg_FPro_products = ?5,
            avg_distance_root = ?6,
            ingred_normalization_term = ?7,
            semantic_tree_name = ?8,
            semantic_tree_node = ?9
        WHERE id = ?1",
        params![
            id,
            name,
            count_products,
            ingred_FPro,
            avg_FPro_products,
            avg_distance_root,
            ingred_normalization_term,
            semantic_tree_name,
            semantic_tree_node
        ],
    )?;
    Ok(())
}

pub fn delete_general_item(id: i32) -> Result<(), Box<dyn Error>> {
    // let conn = Connection::open(DATABASE_FILE)?;
    // conn.execute("DELETE FROM GroceryDB WHERE id = ?1", params![id])?;
    // Ok(())
    let conn = Connection::open(DATABASE_FILE)?;
    
    // Check if a row with the specified ID exists
    let row_count = conn.query_row("SELECT COUNT(*) FROM GroceryDB WHERE id = ?1", params![id], |row| {
        row.get::<usize, i32>(0)
    })?;

    if row_count == 0 {
        // No rows found, return an error
        return Err("No row with the specified ID found".into());
    }
    // Delete the row
    conn.execute("DELETE FROM GroceryDB WHERE id = ?1", params![id])?;
    Ok(())

}

