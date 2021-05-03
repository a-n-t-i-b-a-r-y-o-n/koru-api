/// Helper functions for working with the SQLite DB

use koru;
use crate::serialization::*;

use std::fmt::Error;
use sqlx::{sqlite::*, Sqlite, Pool};


// TODO: Replace full hardcoded path with routine for locating DB
const DB_FILE: &str = "target/debug/koru-api.db";

// Internal connection method
async fn connect() -> Result<Pool<Sqlite>, Error> {
    // Create a connection to the DB and begin a transaction
    let connection = SqlitePool::connect(DB_FILE).await.unwrap();
    Ok(connection)
}

/// Get Device by id
pub async fn get_device(id: i32) -> Option<koru::Device> {
    let mut output = None;
    // Get a connection
    let connection = connect().await;
    if let Ok(connection) = connection {
        // Query for the device id
        let result = sqlx::query("SELECT * FROM device WHERE id=?;")
            .bind(id)
            .fetch_one(&connection)
            .await
            .map(|row: SqliteRow| {
                output = Some(row.to_koru_device())
            });
        match result {
            Ok(_) => output,
            Err(_) => None
        }
    } else {
        println!("[!] Error creating database connection");
        None
    }
}

/* Debug */
pub async fn get_all_devices() -> Option<Vec<koru::Device>> {
    // Vec of Device objects
    let mut output = Vec::<koru::Device>::new();
    // Get a connection
    let connection = connect().await;
    if let Ok(connection) = connection {
        // Get all devices
        let _ = sqlx::query("SELECT * FROM device ORDER BY id;")
            .fetch_all(&connection)
            .await
            .map(|rows: Vec<SqliteRow>| {
                // Consume the returned Vec and map each row
                rows.into_iter()
                    .for_each(|row| {
                        // Push new Device to output Vec based on row
                        output.push(row.to_koru_device());
                    })
            });
        Some(output)
    } else {
        None
    }
}

#[allow(unused_attributes)]
#[cfg_attr(target_os = "linux",)]
use std::process::Command;

#[allow(unused_attributes)]
#[allow(dead_code)]
#[cfg_attr(target_os = "linux",)]
pub async fn create_database_file(path: &str) -> bool {
    const CREATION_SQL: &str = "CREATE TABLE device ( id INT PRIMARY KEY NOT NULL, ipv4 VARCHAR(15) NOT NULL, port INT NOT NULL, name TEXT, network VARCHAR(8) NOT NULL, mac_wlan VARCHAR(17), mac_eth VARCHAR(17) );";
    match Command::new(format!("sqlite3 {} \"{}\"", path, CREATION_SQL)).output() {
        Ok(_) => true,
        Err(_) => false
    }
}