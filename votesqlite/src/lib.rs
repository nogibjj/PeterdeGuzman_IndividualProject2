use zip::read::ZipArchive;
//use encoding::all::WINDOWS_1252;
//use encoding::{DecoderTrap, Encoding};
//use polars::prelude::*;
use reqwest::blocking::get;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
//use std::io::{BufReader, Cursor, Read};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

//Extract functions
//extract function for polling place data

pub fn extract(url: &str, directory: &str) -> Result<(), Box<dyn Error>> {
    // Send GET request
    let response = get(url)?;

    // Check if the request was successful
    if response.status().is_success() {
        // Create directory if it doesn't exist
        std::fs::create_dir_all(directory)?;

        // Get the filename from the URL
        let filename = url.split('/').last().unwrap_or("downloaded.csv");
        let file_path = Path::new(directory).join(filename);

        // Create the file
        let mut file = File::create(&file_path)?;

        // Write the content to the file
        let bytes = response.bytes()?;
        file.write_all(&bytes)?;

        // Return Ok with no value
        Ok(())
    } else {
        // Create a custom error message for failed requests
        let error_message = format!(
            "Failed to download file from {}: Status code {}",
            url,
            response.status()
        );
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}

pub fn extract_zip(url: &str, directory: &str) -> Result<(), Box<dyn Error>> {
    // Creating directory if not present
    let path = Path::new(directory);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // Creating filepath for zipped file
    let zip_filepath = path.join("downloaded_file.zip");

    // Downloading zipped file
    let response =
        get(url).map_err(|e| Box::new(io::Error::new(io::ErrorKind::Other, e.to_string())))?;
    let mut file = File::create(&zip_filepath)?;
    file.write_all(
        &response
            .bytes()
            .map_err(|e| Box::new(io::Error::new(io::ErrorKind::Other, e.to_string())))?,
    )?;

    // Extracting zip file
    let file = File::open(&zip_filepath)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(path)?;

    // Removing zipped file after extraction
    fs::remove_file(zip_filepath)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_extract_zip() -> Result<(), Box<dyn Error>> {
        // Test URL pointing to a small ZIP file for testing
        let test_url = "https://s3.amazonaws.com/dl.ncsbe.gov/data/ncvoter32.zip"; // Replace with a valid ZIP URL
        let test_directory = "test_dir";

        // Create the test directory if it doesn't exist
        if Path::new(test_directory).exists() {
            fs::remove_dir_all(test_directory)?; // Clean up old test directory
        }

        // Run the extract_zip function
        let result = extract_zip(test_url, test_directory);

        // Assert that the function executed successfully
        assert!(result.is_ok());

        // Check if the extracted files exist
        let extracted_file_path = Path::new(test_directory).join("ncvoter32.txt"); // Replace with the expected extracted file name
        assert!(extracted_file_path.exists());

        // Clean up the test directory after the test
        fs::remove_dir_all(test_directory)?;

        Ok(())
    }
}

//Directory Management Functions
pub fn get_county_name(file_name: &str) -> Result<&'static str, String> {
    // Define the county map inside the function
    let county_map: HashMap<u32, &'static str> =
        [(32, "Durham"), (92, "Wake")].iter().cloned().collect();

    let re = Regex::new(r"(\d+)").map_err(|_| "Failed to create regex".to_string())?;
    let number_part = re
        .captures(file_name)
        .and_then(|caps| caps.get(1))
        .ok_or_else(|| "No number found in file name".to_string())?
        .as_str();

    // Parse the number
    match number_part.parse::<u32>() {
        Ok(number) => {
            if let Some(county_name) = county_map.get(&number).copied() {
                println!("Matched county: {} County", county_name); // Print the matched county name
                Ok(county_name) // Return the matched county name
            } else {
                Err("County not found for the given number".to_string())
                // Return error if county not found
            }
        }
        Err(_) => {
            Err("Failed to parse the number".to_string()) // Return parsing error
        }
    }
}

#[cfg(test)]
mod tests_countyname {
    use super::*;

    #[test]
    fn test_get_county_name() {
        //Building a test case for Durham County
        let file_name = "ncvoter32.txt";
        //Call function
        let result = get_county_name(file_name);
        //Assert that result is Ok and matches "Durham"
        assert_eq!(result, Ok("Durham"));
    }

    #[test]
    fn test_get_county_name_invalid() {
        //Testing for file names that are invalid
        let file_name = "ncvotingdata0.txt";
        //Call function
        let result = get_county_name(file_name);
        //Assert that Result is the expected error
        assert_eq!(
            result,
            Err("County not found for the given number".to_string())
        );
    }
}

pub fn print_county_names_in_directory(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        match get_county_name(&file_name_str) {
            Ok(county_name) => println!("{}", county_name),
            Err(e) => eprintln!("Error processing file '{}': {}", file_name_str, e),
        }
    }

    Ok(())
}
