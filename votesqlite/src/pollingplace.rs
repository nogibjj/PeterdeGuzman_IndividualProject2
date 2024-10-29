//Functions for North Carolina Polling Place Data

//use csv::ReaderBuilder;
use csv::ReaderBuilder;
use encoding_rs::UTF_16LE;
use rusqlite::types::Value;
use rusqlite::{params, Connection, Result, Row};
use std::error::Error;

use jemallocator::Jemalloc;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;
use sysinfo::System;

//Load
// pub fn load_pollingplace(
//     conn: &Connection,
//     table_name: &str,
//     file_path: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     let insert_query = format!(
//         "INSERT INTO {} (election_dt,
//             county_name,
//             polling_place_id,
//             polling_place_name,
//             precinct_name,
//             house_num,
//             street_name,
//             city,
//             state,
//             zip) VALUES (?,?,?,?,?,?,?,?,?,?)",
//         table_name
//     );
//     //loop to set schema
//     for result in rdr.records() {
//         let record = result?;
//         let election_dt: &str = &record[0];
//         let county_name: &str = &record[1];
//         let polling_place_id: i32 = record[2].parse()?;
//         let polling_place_name: &str = &record[3];
//         let precinct_name: &str = &record[4];
//         let house_num: &str = &record[5];
//         let street_name: &str = &record[6];
//         let city: &str = &record[7];
//         let state: &str = &record[8];
//         let zip: &str = &record[9];
//         conn.execute(
//             &insert_query,
//             params![
//                 election_dt,
//                 county_name,
//                 polling_place_id,
//                 polling_place_name,
//                 precinct_name,
//                 house_num,
//                 street_name,
//                 city,
//                 state,
//                 zip,
//             ],
//         )?;
//     }
//     println!(
//         "Data loaded successfully from '{}' into table '{}'.",
//         file_path, table_name
//     );
//     Ok(())
// }

pub fn load_pollingplace(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Open the dataset file
    let file = File::open(file_path)?;

    // Create a BufReader to read the file
    let mut reader = BufReader::new(file);

    // Read the file contents into a Vec<u8>
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    // Decode the contents from UTF-16LE
    let (decoded, _, had_errors) = UTF_16LE.decode(&buf);
    if had_errors {
        return Err(Box::from("Failed to decode UTF-16LE data."));
    }

    // Split into lines and clean up
    let payload: Vec<String> = decoded
        .lines()
        .map(|line| line.replace("\0", "")) // Remove null bytes
        .collect();

    // Generate new table for the database
    conn.execute(
        format!("DROP TABLE IF EXISTS {}", table_name).as_str(),
        params![],
    )?;

    conn.execute(
        format!(
            r#"
            CREATE TABLE {} (
                election_dt DATE,
                county_name TEXT,
                polling_place_id INTEGER,
                polling_place_name TEXT,
                precinct_name TEXT,
                house_num INTEGER,
                street_name TEXT,
                city TEXT,
                state TEXT,
                zip TEXT
            )
            "#,
            table_name
        )
        .as_str(),
        params![],
    )?;

    // Create a CSV reader for the collected payload
    let binding = payload.join("\n");
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t') // Use tab as the delimiter
        .has_headers(true) // Skip the header row
        .from_reader(binding.as_bytes());

    // Insert values
    for result in rdr.records() {
        let record = result?;

        // Ensure the record has at least 10 fields
        if record.len() < 10 {
            return Err(Box::from(format!(
                "Record has insufficient fields: expected 10 but got {}",
                record.len()
            )));
        }

        conn.execute(
            format!(
                r#"
                INSERT INTO {} (
                    election_dt,
                    county_name,
                    polling_place_id,
                    polling_place_name,
                    precinct_name,
                    house_num,
                    street_name,
                    city,
                    state,
                    zip
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                table_name
            )
            .as_str(),
            params![
                &record[0],
                &record[1],
                record[2].parse::<i32>()?,
                &record[3],
                &record[4],
                record[5].parse::<i32>()?, // Ensure house_num is parsed as i32
                &record[6],
                &record[7],
                &record[8],
                &record[9],
            ],
        )?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}
//CRUD Operations

//Create Table

pub fn create_pollingplace(conn: &Connection, table_name: &str) -> Result<(), Box<dyn Error>> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
                election_dt TEXT, 
                county_name TEXT,
                polling_place_id INTEGER,
                polling_place_name TEXT,
                precinct_name TEXT,
                house_num INTEGER,
                street_name TEXT,
                city TEXT, 
                state TEXT,
                zip TEXT)",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

//Read or Query
pub fn query_pp(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        let election_dt: String = row.get(0)?;
        let county_name: String = row.get(1)?;
        let polling_place_id: i32 = row.get(2)?;
        let polling_place_name: String = row.get(3)?;
        let precinct_name: String = row.get(4)?;
        let house_num: i32 = row.get(5)?;
        let street_name: String = row.get(6)?;
        let city: String = row.get(7)?;
        let state: String = row.get(8)?;
        let zip: String = row.get(9)?;
        Ok((
            election_dt,
            county_name,
            polling_place_id,
            polling_place_name,
            precinct_name,
            house_num,
            street_name,
            city,
            state,
            zip,
        ))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (
            election_dt,
            county_name,
            polling_place_id,
            polling_place_name,
            precinct_name,
            house_num,
            street_name,
            city,
            state,
            zip,
        ) = row?;
        println!("Election Date: {}, County Name: {}, Polling Place ID: {}, Polling Place Name: {}, Precinct Name: {}, House Num: {}, Street Name: {}, City: {}, State: {}, Zip: {}",             election_dt,
        county_name,
        polling_place_id,
        polling_place_name,
        precinct_name,
        house_num,
        street_name,
        city,
        state,
        zip,);
    }

    Ok(())
}

//Test

// pub fn general_query(conn: &Connection, query: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
//     let mut stmt = conn.prepare(query)?;

//     // Get the number of columns from the statement
//     let column_count = stmt.column_count();

//     let rows = stmt.query_map(params![], |row: &Row| {
//         let mut row_vec = Vec::new();

//         // Collect the column values as strings
//         for i in 0..column_count {
//             // Use `Value::as_string` to handle different types
//             match row.get::<_, Value>(i)? {
//                 Value::Null => row_vec.push("NULL".to_string()), // Handle NULL values
//                 Value::Integer(val) => row_vec.push(val.to_string()), // Convert Integer to String
//                 Value::Real(val) => row_vec.push(val.to_string()), // Convert Real to String
//                 Value::Text(val) => row_vec.push(val),           // Already a String
//                 Value::Blob(_) => row_vec.push("BLOB".to_string()), // Handle BLOBs, you may want to process them differently
//             }
//         }
//         Ok(row_vec)
//     })?;

//     // Collect the results into a vector of vectors
//     let result: Vec<Vec<String>> = rows.filter_map(Result::ok).collect();

//     // Print the results
//     for row in &result {
//         println!("{:?}", row);
//     }

//     Ok(result)
// }

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub fn general_query(conn: &Connection, query: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    // Start timing
    let start_time = Instant::now();

    let mut stmt = conn.prepare(query)?;

    // Get the number of columns from the statement
    let column_count = stmt.column_count();

    let rows = stmt.query_map(params![], |row: &Row| {
        let mut row_vec = Vec::new();

        // Collect the column values as strings
        for i in 0..column_count {
            match row.get::<_, Value>(i)? {
                Value::Null => row_vec.push("NULL".to_string()),
                Value::Integer(val) => row_vec.push(val.to_string()),
                Value::Real(val) => row_vec.push(val.to_string()),
                Value::Text(val) => row_vec.push(val),
                Value::Blob(_) => row_vec.push("BLOB".to_string()),
            }
        }
        Ok(row_vec)
    })?;

    // Collect the results into a vector of vectors
    let result: Vec<Vec<String>> = rows.filter_map(Result::ok).collect();

    // Print the results
    for row in &result {
        println!("{:?}", row);
    }

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();

    // Track memory usage
    let mut sys = System::new_all();
    sys.refresh_all();
    let memory_usage = sys.total_memory() - sys.available_memory(); // Used memory in bytes

    // Print time and memory usage
    println!("Time taken: {:?}", elapsed_time);
    println!("Memory allocated: {} bytes", memory_usage);

    Ok(result)
}
