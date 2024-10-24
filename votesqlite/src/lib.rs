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

// pub fn extract_zip(url: &str, directory: &str) -> io::Result<String> {
//     // Creating directory if not present
//     let path = Path::new(directory);
//     if !path.exists() {
//         fs::create_dir_all(path)?;
//     }

//     // Creating filepath for zipped file
//     let zip_filepath = path.join("downloaded_file.zip");

//     // Downloading zipped file
//     let response = get(url).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
//     let mut file = File::create(&zip_filepath)?;
//     file.write_all(
//         &response
//             .bytes()
//             .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?,
//     )?;

//     // Extracting zip file
//     let file = File::open(&zip_filepath)?;
//     let mut archive = ZipArchive::new(file)?;
//     archive.extract(path)?;

//     // Removing zipped file after extraction
//     fs::remove_file(zip_filepath)?;

//     Ok(directory.to_string())
// }

// pub fn get_county_name(file_name: &str) -> Option<&'static str> {
//     // Define the county map inside the function
//     let county_map: HashMap<u32, &str> = [(32, "Durham"), (92, "Wake")].iter().cloned().collect();

//     // Extract the number from the file name (assuming the format is "vote<number>.txt")
//     let number_part = file_name.strip_prefix("vote")?.strip_suffix(".txt")?;

//     // Parse the number
//     match number_part.parse::<u32>() {
//         Ok(number) => county_map.get(&number).copied(),
//         Err(_) => None,
//     }
// }

// pub fn get_county_name(file_name: &str) -> Result<&'static str, String> {
//     // Define the county map inside the function
//     let county_map: HashMap<u32, &'static str> =
//         [(32, "Durham"), (92, "Wake")].iter().cloned().collect();

//     // Extract the number from the file name (assuming the format is "vote<number>.txt")
//     let number_part = file_name
//         .strip_prefix("ncvoter")
//         .ok_or("Invalid file name format")?
//         .strip_suffix(".txt")
//         .ok_or("Invalid file name format")?;

//     // Parse the number
//     match number_part.parse::<u32>() {
//         Ok(number) => county_map
//             .get(&number)
//             .copied()
//             .ok_or("County not found".to_string()), // Return an error if county is not found
//         Err(_) => Err("Failed to parse the number".to_string()),
//     }
//     print()
// }

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

// pub fn transform_voterreg(
//     txtfile: &str,
//     directory: &str,
//     county: &str,
//     date: &str,
// ) -> io::Result<()> {
//     // Read the TSV file with Windows-1252 encoding
//     let file = File::open(txtfile)?;
//     let mut reader = BufReader::new(file);
//     let mut buffer = Vec::new();
//     reader.read_to_end(&mut buffer)?;

//     // Decode from Windows-1252 to UTF-8
//     let decoded = WINDOWS_1252
//         .decode(&buffer, DecoderTrap::Strict)
//         .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

//     // Create a cursor
//     let cursor = Cursor::new(decoded.into_bytes());

//     // Create a DataFrame from the decoded string
//     let df = CsvReader::new(cursor)
//         .delimiter(b'\t')
//         .has_header(true)
//         .finish()
//         .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

//     // Expected columns
//     let expected_columns = vec![
//         "county_id",
//         "county_desc",
//         "voter_reg_num",
//         "ncid",
//         "last_name",
//         "first_name",
//         "middle_name",
//         "name_suffix_lbl",
//         "status_cd",
//         "voter_status_desc",
//         "reason_cd",
//         "voter_status_reason_desc",
//         "res_street_address",
//         "res_city_desc",
//         "state_cd",
//         "zip_code",
//         "mail_addr1",
//         "mail_addr2",
//         "mail_addr3",
//         "mail_addr4",
//         "mail_city",
//         "mail_state",
//         "mail_zipcode",
//         "full_phone_number",
//         "confidential_ind",
//         "registr_dt",
//         "race_code",
//         "ethnic_code",
//         "party_cd",
//         "gender_code",
//         "birth_year",
//         "age_at_year_end",
//         "birth_state",
//         "drivers_lic",
//         "precinct_abbrv",
//         "precinct_desc",
//         "municipality_abbrv",
//         "municipality_desc",
//         "ward_abbrv",
//         "ward_desc",
//         "cong_dist_abbrv",
//         "super_court_abbrv",
//         "judic_dist_abbrv",
//         "nc_senate_abbrv",
//         "nc_house_abbrv",
//         "county_commiss_abbrv",
//         "county_commiss_desc",
//         "township_abbrv",
//         "township_desc",
//         "school_dist_abbrv",
//         "school_dist_desc",
//         "fire_dist_abbrv",
//         "fire_dist_desc",
//         "water_dist_abbrv",
//         "water_dist_desc",
//         "sewer_dist_abbrv",
//         "sewer_dist_desc",
//         "sanit_dist_abbrv",
//         "sanit_dist_desc",
//         "rescue_dist_abbrv",
//         "rescue_dist_desc",
//         "munic_dist_abbrv",
//         "munic_dist_desc",
//         "dist_1_abbrv",
//         "dist_1_desc",
//         "vtd_abbrv",
//         "vtd_desc",
//     ];

//     // Confirming data has expected columns
//     if df.get_column_names().len() == expected_columns.len()
//         && expected_columns
//             .iter()
//             .all(|col| df.get_column(col).is_ok())
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", df.get_column_names());
//     }

//     // Save as UTF-16 encoded CSV
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let mut file = fs::File::create(filepath)?;

//     let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

//     // Write the dataframe to the CSV file
//     for row in df.iter_rows() {
//         writer.write_record(row.iter().map(|s| s.to_string()))?;
//     }

//     writer.flush()?;

//     Ok(())
// }
