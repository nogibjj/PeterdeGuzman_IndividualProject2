use crate::transform::transform_voterreg;
use clap::{Parser, Subcommand};
use std::result::Result;
use transform::remove_invalid_utf8_bytes;
use votesqlite::{extract_zip, get_county_name, print_county_names_in_directory};
mod transform;

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
    #[command(alias = "remove_invalid_utf8_bytes", long_flag = "remove_invalidutf8")]
    RemoveInvalidUTF8 {
        input_file: String,
        output_file: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Parse CLI arguments and store them in the args object
    let args = Cli::parse();

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
    }
    Ok(())
}
