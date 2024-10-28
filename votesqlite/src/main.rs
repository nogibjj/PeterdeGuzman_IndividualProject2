use crate::transform::{create_table, transform_voterreg};
use clap::{Command, Parser, Subcommand};
use std::{fs::create_dir, result::Result};
use transform::{drop_table, load_voterreg, query_exec, remove_invalid_utf8_bytes};
use votesqlite::{extract_zip, get_county_name, print_county_names_in_directory};
mod transform;
use rusqlite::{Connection, Result};

/// A simple CLI tool to download and extract ZIP files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//This struct will generate an object from our CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
// I separate the commands as enum types
enum Commands {
    //Download a file from a link, unzip, and save to directory
    #[command(alias = "extract_zip", short_flag = 'z', long_flag = "extract_zip")]
    ExtractZipped { url: String, directory: String },
    //Get county name from a file name
    #[command(alias = "get_county_name", long_flag = "get_county_name")]
    GetCountyName { file_name: String },
    //Print the county name of all files in a directory
    #[command(alias = "print_county_names", long_flag = "print_county_names")]
    PrintCountyNames { path: String },
    #[command(alias = "transform_voterreg", long_flag = "transform_voterreg")]
    TransformVoterReg {
        txtfile: String,
        county: String,
        date: String,
        directory: String,
    },
    //Remove Invalid UTF-8 Bytes from file
    #[command(alias = "remove_invalid_utf8_bytes", long_flag = "remove_invalidutf8")]
    RemoveInvalidUTF8 {
        input_file: String,
        output_file: String,
    },
    //Load NCSBE voter registration data
    #[command(alias = "load_voterreg", long_flag = "load_vr")]
    LoadVoterReg {
        table_name: String,
        file_path: String,
    },
    //Create
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    //Read or Query
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },
    //Update
    //Delete table
    #[command(alias = "d", short_flag = 'd')]
    Delete { delete_query: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Parse CLI arguments and store them in the args object
    let args = Cli::parse();

    //generate connection
    let conn = Connection::open("voterreg_durham.db")?;

    //Match the behavior on the subcommand and call lib functions
    match args.command {
        Commands::ExtractZipped { url, directory } => {
            println!(
                "Downloading file from {} and unzipping file to {}",
                url, directory
            );
            extract_zip(&url, &directory).expect("Failed to extract zipped file")
        }
        Commands::GetCountyName { file_name } => {
            println!("Getting county name from {}.", file_name);
            get_county_name(&file_name).expect("County name not found.");
        }
        Commands::PrintCountyNames { path } => {
            print_county_names_in_directory(&path).expect("Did not identify any county names.")
        }
        Commands::TransformVoterReg {
            txtfile,
            county,
            date,
            directory,
        } => {
            println!("Transforming file {} into UTF-16.", txtfile);
            transform_voterreg(&txtfile, &county, &date, &directory)
                .expect("Did not successfully transform dataset.")
        }
        Commands::RemoveInvalidUTF8 {
            input_file,
            output_file,
        } => {
            println!(
                "Removing Invalid UTF-8 bytes from {} and saving to {}.",
                input_file, output_file
            );
            remove_invalid_utf8_bytes(&input_file, &output_file)
                .expect("Removing Invalid UTF-8 bytes failed.")
        }
        Commands::LoadVoterReg {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_voterreg(&conn, &table_name, &file_path).expect("Failed to load data from csv");
        }
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { query } => {
            println!("Executing query: {}", query);
            query_exec(&conn, &query).expect("Failed to execute query");
        }
        Commands::Delete { delete_query } => {
            println!("Deleting {}", delete_query);
            drop_table(&conn, &delete_query).expect("Failed to drop table");
        }
    }
    Ok(())
}
