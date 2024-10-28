use rusqlite::{Connection, Result};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

//Transform Functions

pub fn remove_invalid_utf8_bytes(
    input_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Open the input file for reading
    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    // Open the output file for writing
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    // Read the file as bytes
    for byte in reader.bytes() {
        match byte {
            Ok(b) => {
                // Attempt to write the byte; it will only be valid UTF-8
                if b.is_ascii() || (0xC2..=0xF4).contains(&b) {
                    // Basic check for valid UTF-8 leading bytes
                    writer.write_all(&[b])?;
                }
            }
            Err(e) => {
                eprintln!("Error reading byte: {:?}", e);
            }
        }
    }

    // Flush and close the writer
    writer.flush()?;
    println!("Invalid UTF-8 bytes removed and saved to {:?}", output_file);

    Ok(())
}

//Load Functions

//CRUD Operations

//Create table with the new schema

//Read data from the table

//Update data in the table
pub fn update_table(
    conn: &Connection,
    table_name: &str,
    set_clause: &str,
    condition: &str,
) -> Result<()> {
    // Build a SQL UPDATE query using arguments for table name, set clause, and condition
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );

    // Execute the update query
    let affected_rows = conn.execute(&update_query, [])?;

    // Output the number of rows updated
    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );

    Ok(())
}

//Delete the table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}
