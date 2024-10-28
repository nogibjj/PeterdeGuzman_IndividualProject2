//Functions for North Carolina Polling Place Data

use rusqlite::{Connection, Result};
use std::error::Error;

//Load

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
        // Assuming the `users` table has an `id` and `name` column
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
