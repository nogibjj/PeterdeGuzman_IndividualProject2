// use std::error::Error;
// use std::fs::File;
// use std::io::{self, BufReader};
// use csv::ReaderBuilder;
// use sqlx::sqlite::SqlitePoolOptions;
// use sql::{Connection, Executor};// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;
//         // Convert each field from Windows-1252 to UTF-16 and write to the new file
//         let utf16_record: Vec<String> = record
//             .iter()
//             .map(|field| {
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
//                 let decoded_str = String::from_utf8(bytes).unwrap(); // Decode bytes to String
//                 let _utf16: Vec<u16> = decoded_str.encode_utf16().collect(); // Convert String to UTF-16

//                 // Convert the u16 vector back to String if necessary or return the UTF-16 representation
//                 // Here we're returning the decoded string directly since you are writing to CSV.
//                 decoded_str
//             })
//             .collect();

//         writer.write_record(&utf16_record)?;
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }

//Loading Polling Places

// #[derive(Debug)]
// struct PollingPlace {
//     election_dt: String,
//     county_name: String,
//     polling_place_id: i32,
//     polling_place_name: String,
//     precinct_name: String,
//     house_num: Option<i32>,
//     street_name: String,
//     city: String,
//     state: String,
//     zip: String,
// }

// //Load Polling Places data
// pub fn load_pollingplaces(dataset: &str, year: &str) -> Result<String, Box<dyn Error>> {
//     //Read CSV
//     let file = File::open(dataset)?;

//     //Connect to SQLite db

//     //Create table

//     //Insert values into db
// }


// pub fn remove_invalid_utf8_bytes(
//     input_file: &str,
//     output_file: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file for reading
//     let input = File::open(input_file)?;
//     let reader = BufReader::new(input);

//     // Open the output file for writing
//     let output = File::create(output_file)?;
//     let mut writer = BufWriter::new(output);

//     // Read the file as bytes
//     for byte in reader.bytes() {
//         match byte {
//             Ok(b) => {
//                 // Attempt to write the byte; it will only be valid UTF-8
//                 if b.is_ascii() || (0xC2..=0xF4).contains(&b) {
//                     // Basic check for valid UTF-8 leading bytes
//                     writer.write_all(&[b])?;
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error reading byte: {:?}", e);
//             }
//         }
//     }

//     // Flush and close the writer
//     writer.flush()?;
//     println!("Invalid UTF-8 bytes removed and saved to {:?}", output_file);

//     Ok(())
// }



//Transform Windows-1521 encoded files to UTF-16
// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;

//         // Convert each field from Windows-1252 to UTF-8 and write to the new file
//         let utf16_record: Vec<String> = record
//             .iter()
//             .map(|field| {
//                 // Encode to bytes using Windows-1252
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();

//                 // Decode bytes to String with error handling
//                 String::from_utf8(bytes).unwrap_or_else(|err| {
//                     eprintln!("Error decoding field: {:?}", err);
//                     // Return a placeholder for invalid UTF-8 bytes
//                     String::from("INVALID_UTF8")
//                 })
//             })
//             .collect();

//         writer.write_record(&utf16_record)?;
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }

// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;

//         // Try to convert each field from Windows-1252 to UTF-8
//         let utf16_record: Vec<Option<String>> = record
//             .iter()
//             .map(|field| {
//                 // Encode to bytes using Windows-1252
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
//                 // Attempt to decode bytes to String
//                 String::from_utf8(bytes).map(Some).map_err(|err| {
//                     eprintln!("Error decoding field: {:?}", err);
//                     // Return None for invalid UTF-8

//                 })
//             })
//             .collect();

//         // Check if any fields failed to decode
//         if utf16_record.iter().any(|result| result.is_none()) {
//             // Skip this record if there was any decoding error
//             eprintln!("Skipping record due to invalid UTF-8 bytes.");
//             continue;
//         }

//         // Collect valid results, unwrapping the Option
//         let valid_record: Vec<String> = utf16_record
//             .into_iter()
//             .filter_map(|opt| opt) // Filter out None values
//             .collect();

//         // Write the valid record to the output file
//         writer.write_record(&valid_record)?;
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }

// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;

//         // Try to convert each field from Windows-1252 to UTF-8
//         let utf16_record: Vec<String> = record
//             .iter()
//             .filter_map(|field| {
//                 // Encode to bytes using Windows-1252
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
//                 // Attempt to decode bytes to String
//                 String::from_utf8(bytes)
//                     .map_err(|err| {
//                         eprintln!("Error decoding field: {:?}", err);
//                         // Return None for invalid UTF-8
//                     })
//                     .ok() // Convert Result to Option
//             })
//             .collect();

//         // Check if we have a full record of valid fields
//         if utf16_record.len() != record.len() {
//             // Skip this record if there was any decoding error
//             eprintln!("Skipping record due to invalid UTF-8 bytes.");
//             continue;
//         }

//         // Write the valid record to the output file
//         writer.write_record(&utf16_record)?;
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }




// Below here is code removed by Peter on 10/28/24

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


// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;

//         // Try to convert each field from Windows-1252 to UTF-8
//         let utf16_record: Vec<String> = record
//             .iter()
//             .filter_map(|field| {
//                 // Encode to bytes using Windows-1252
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
//                 // Attempt to decode bytes to String
//                 String::from_utf8(bytes)
//                     .map_err(|err| {
//                         eprintln!("Error decoding field: {:?}", err);
//                         // Return None for invalid UTF-8
//                     })
//                     .ok() // Convert Result to Option
//             })
//             .collect();

//         // Check if we have a full record of valid fields
//         if utf16_record.len() == record.len() {
//             // Only write valid records to the output file
//             writer.write_record(&utf16_record)?;
//         } else {
//             // Skip this record if there was any decoding error
//             eprintln!("Skipping record due to invalid UTF-8 bytes.");
//         }
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }

// pub fn transform_voterreg(
//     txtfile: &str,
//     county: &str,
//     date: &str,
//     directory: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Open the input file
//     let file = File::open(txtfile)?;
//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

//     // Check the expected columns
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

//     // Check if the columns match
//     let headers = reader.headers()?;
//     let actual_columns: Vec<&str> = headers.iter().collect();

//     if actual_columns.len() == 67
//         && expected_columns
//             .iter()
//             .all(|col| actual_columns.contains(col))
//     {
//         println!("All expected columns are present in dataframe.");
//     } else {
//         println!("The CSV does not have the correct columns.");
//         println!("Expected columns: {:?}", expected_columns);
//         println!("Actual columns: {:?}", actual_columns);
//         return Err("Column mismatch".into());
//     }

//     // Prepare the output file path
//     let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
//     let output_file = File::create(&filepath)?;
//     let mut writer = WriterBuilder::new()
//         .delimiter(b'\t')
//         .from_writer(BufWriter::new(output_file));

//     // Write the headers to the output file
//     writer.write_record(&actual_columns)?;

//     // Read and convert records, then write to the output file
//     for result in reader.records() {
//         let record = result?;

//         // Try to convert each field from Windows-1252 to UTF-8
//         let utf16_record: Vec<String> = record
//             .iter()
//             .filter_map(|field| {
//                 // Encode to bytes using Windows-1252
//                 let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
//                 // Attempt to decode bytes to String
//                 String::from_utf8(bytes)
//                     .map_err(|err| {
//                         eprintln!("Error decoding field: {:?}", err);
//                         // Return None for invalid UTF-8
//                     })
//                     .ok() // Convert Result to Option
//             })
//             .collect();

//         // Check if we have a full record of valid fields
//         if utf16_record.len() == record.len() {
//             // Only write valid records to the output file
//             writer.write_record(&utf16_record)?;
//         } else {
//             // Skip this record if there was any decoding error
//             eprintln!("Skipping record due to invalid UTF-8 bytes.");
//         }
//     }

//     writer.flush()?;
//     println!("Data successfully transformed and saved to {:?}", filepath);

//     Ok(())
// }

//Load voter registration dataset

// pub fn load_voterreg(
//     conn: &Connection,
//     table_name: &str,
//     file_path: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     let insert_query = format!(
//         "Insert INTO {} (county_id,
//         county_desc,
//         voter_reg_num,
//         ncid,
//         last_name,
//         first_name,
//         middle_name,
//         name_suffix_lbl,
//         status_cd,
//         voter_status_desc,
//         reason_cd,
//         voter_status_reason_desc,
//         res_street_address,
//         res_city_desc,
//         state_cd,
//         zip_code,
//         mail_addr1,
//         mail_addr2,
//         mail_addr3,
//         mail_addr4,
//         mail_city,
//         mail_state,
//         mail_zipcode,
//         full_phone_number,
//         confidential_ind,
//         registr_dt,
//         race_code,
//         ethnic_code,
//         party_cd,
//         gender_code,
//         birth_year,
//         age_at_year_end,
//         birth_state,
//         drivers_lic,
//         precinct_abbrv,
//         precinct_desc,
//         municipality_abbrv,
//         municipality_desc,
//         ward_abbrv,
//         ward_desc,
//         cong_dist_abbrv,
//         super_court_abbrv,
//         judic_dist_abbrv,
//         nc_senate_abbrv,
//         nc_house_abbrv,
//         county_commiss_abbrv,
//         county_commiss_desc,
//         township_abbrv,
//         township_desc,
//         school_dist_abbrv,
//         school_dist_desc,
//         fire_dist_abbrv,
//         fire_dist_desc,
//         water_dist_abbrv,
//         water_dist_desc,
//         sewer_dist_abbrv,
//         sewer_dist_desc,
//         sanit_dist_abbrv,
//         sanit_dist_desc,
//         rescue_dist_abbrv,
//         rescue_dist_desc,
//         munic_dist_abbrv,
//         munic_dist_desc,
//         dist_1_abbrv,
//         dist_1_desc,
//         vtd_abbrv,
//         vtd_desc) (?, ?, ?)",
//         table_name
//     );
//     // for loop to set schema
//     for result in rdr.records() {
//         let record = result?;
//         let county_id: i32 = record[0].parse()?;
//         let county_desc: &str = &record[1];
//         let voter_reg_num: &str = &record[2];
//         let ncid: &str = &record[3];
//         let last_name: &str = &record[4];
//         let first_name: &str = &record[5];
//         let middle_name: &str = &record[6];
//         let name_suffix_lbl: &str = &record[7];
//         let status_cd: &str = &record[8];
//         let voter_status_desc: &str = &record[9];
//         let reason_cd: &str = &record[10];
//         let voter_status_reason_desc: &str = &record[11];
//         let res_street_address: &str = &record[12];
//         let res_city_desc: &str = &record[13];
//         let state_cd: &str = &record[14];
//         let zip_code: &str = &record[15];
//         let mail_addr1: &str = &record[16];
//         let mail_addr2: &str = &record[17];
//         let mail_addr3: &str = &record[18];
//         let mail_addr4: &str = &record[19];
//         let mail_city: &str = &record[20];
//         let mail_state: &str = &record[21];
//         let mail_zipcode: &str = &record[22];
//         let full_phone_number: &str = &record[23];
//         let confidential_ind: &str = &record[24];
//         let registr_dt: &str = &record[25];
//         let race_code: &str = &record[26];
//         let ethnic_code: &str = &record[27];
//         let party_cd: &str = &record[28];
//         let gender_code: &str = &record[29];
//         let birth_year: i32 = record[30].parse()?;
//         let age_at_year_end: &str = &record[31];
//         let birth_state: &str = &record[32];
//         let drivers_lic: &str = &record[33];
//         let precinct_abbrv: &str = &record[34];
//         let precinct_desc: &str = &record[35];
//         let municipality_abbrv: &str = &record[36];
//         let municipality_desc: &str = &record[37];
//         let ward_abbrv: &str = &record[38];
//         let ward_desc: &str = &record[39];
//         let cong_dist_abbrv: &str = &record[40];
//         let super_court_abbrv: &str = &record[41];
//         let judic_dist_abbrv: &str = &record[42];
//         let nc_senate_abbrv: &str = &record[43];
//         let nc_house_abbrv: &str = &record[44];
//         let county_commiss_abbrv: &str = &record[45];
//         let county_commiss_desc: &str = &record[46];
//         let township_abbrv: &str = &record[47];
//         let township_desc: &str = &record[48];
//         let school_dist_abbrv: &str = &record[49];
//         let school_dist_desc: &str = &record[50];
//         let fire_dist_abbrv: &str = &record[51];
//         let fire_dist_desc: &str = &record[52];
//         let water_dist_abbrv: &str = &record[53];
//         let water_dist_desc: &str = &record[54];
//         let sewer_dist_abbrv: &str = &record[55];
//         let sewer_dist_desc: &str = &record[56];
//         let sanit_dist_abbrv: &str = &record[57];
//         let sanit_dist_desc: &str = &record[58];
//         let rescue_dist_abbrv: &str = &record[59];
//         let rescue_dist_desc: &str = &record[60];
//         let munic_dist_abbrv: &str = &record[61];
//         let munic_dist_desc: &str = &record[62];
//         let dist_1_abbrv: &str = &record[63];
//         let dist_1_desc: &str = &record[64];
//         let vtd_abbrv: &str = &record[65];
//         let vtd_description: &str = &record[66];

//         conn.execute(
//             &insert_query,
//             params![
//                 county_id,
//                 county_desc,
//                 voter_reg_num,
//                 ncid,
//                 last_name,
//                 first_name,
//                 middle_name,
//                 name_suffix_lbl,
//                 status_cd,
//                 voter_status_desc,
//                 reason_cd,
//                 voter_status_reason_desc,
//                 res_street_address,
//                 res_city_desc,
//                 state_cd,
//                 zip_code,
//                 mail_addr1,
//                 mail_addr2,
//                 mail_addr3,
//                 mail_addr4,
//                 mail_city,
//                 mail_state,
//                 mail_zipcode,
//                 full_phone_number,
//                 confidential_ind,
//                 registr_dt,
//                 race_code,
//                 ethnic_code,
//                 party_cd,
//                 gender_code,
//                 birth_year,
//                 age_at_year_end,
//                 birth_state,
//                 drivers_lic,
//                 precinct_abbrv,
//                 precinct_desc,
//                 municipality_abbrv,
//                 municipality_desc,
//                 ward_abbrv,
//                 ward_desc,
//                 cong_dist_abbrv,
//                 super_court_abbrv,
//                 judic_dist_abbrv,
//                 nc_senate_abbrv,
//                 nc_house_abbrv,
//                 county_commiss_abbrv,
//                 county_commiss_desc,
//                 township_abbrv,
//                 township_desc,
//                 school_dist_abbrv,
//                 school_dist_desc,
//                 fire_dist_abbrv,
//                 fire_dist_desc,
//                 water_dist_abbrv,
//                 water_dist_desc,
//                 sewer_dist_abbrv,
//                 sewer_dist_desc,
//                 sanit_dist_abbrv,
//                 sanit_dist_desc,
//                 rescue_dist_abbrv,
//                 rescue_dist_desc,
//                 munic_dist_abbrv,
//                 munic_dist_desc,
//                 dist_1_abbrv,
//                 dist_1_desc,
//                 vtd_abbrv,
//                 vtd_description
//             ],
//         )?;
//     }

//     println!(
//         "Data was successfully loaded from '{}' into table '{}'.",
//         file_path, table_name
//     );
//     Ok(())
// }

//Test lossy

// pub fn load_voterreg(
//     conn: &Connection,
//     table_name: &str,
//     file_path: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     let insert_query = format!(
//         "INSERT INTO {} (
//             county_id,
//             county_desc,
//             voter_reg_num,
//             ncid,
//             last_name,
//             first_name,
//             middle_name,
//             name_suffix_lbl,
//             status_cd,
//             voter_status_desc,
//             reason_cd,
//             voter_status_reason_desc,
//             res_street_address,
//             res_city_desc,
//             state_cd,
//             zip_code,
//             mail_addr1,
//             mail_addr2,
//             mail_addr3,
//             mail_addr4,
//             mail_city,
//             mail_state,
//             mail_zipcode,
//             full_phone_number,
//             confidential_ind,
//             registr_dt,
//             race_code,
//             ethnic_code,
//             party_cd,
//             gender_code,
//             birth_year,
//             age_at_year_end,
//             birth_state,
//             drivers_lic,
//             precinct_abbrv,
//             precinct_desc,
//             municipality_abbrv,
//             municipality_desc,
//             ward_abbrv,
//             ward_desc,
//             cong_dist_abbrv,
//             super_court_abbrv,
//             judic_dist_abbrv,
//             nc_senate_abbrv,
//             nc_house_abbrv,
//             county_commiss_abbrv,
//             county_commiss_desc,
//             township_abbrv,
//             township_desc,
//             school_dist_abbrv,
//             school_dist_desc,
//             fire_dist_abbrv,
//             fire_dist_desc,
//             water_dist_abbrv,
//             water_dist_desc,
//             sewer_dist_abbrv,
//             sewer_dist_desc,
//             sanit_dist_abbrv,
//             sanit_dist_desc,
//             rescue_dist_abbrv,
//             rescue_dist_desc,
//             munic_dist_abbrv,
//             munic_dist_desc,
//             dist_1_abbrv,
//             dist_1_desc,
//             vtd_abbrv,
//             vtd_desc
//         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
//         table_name
//     );

//     for result in rdr.records() {
//         let record = result?;

//         // Safely parse county_id and birth_year with error handling
//         let county_id: Result<i32, _> = record[0].parse();
//         let birth_year: Result<i32, _> = record[30].parse();

// if let (Ok(county_id), Ok(birth_year)) = (county_id, birth_year) {
//     let county_desc = String::from_utf8_lossy(record[1].as_bytes()).to_string();
//     let voter_reg_num = String::from_utf8_lossy(record[2].as_bytes()).to_string();
//     let ncid = String::from_utf8_lossy(record[3].as_bytes()).to_string();
//     let last_name = String::from_utf8_lossy(record[4].as_bytes()).to_string();
//     let first_name = String::from_utf8_lossy(record[5].as_bytes()).to_string();
//     let middle_name = String::from_utf8_lossy(record[6].as_bytes()).to_string();
//     let name_suffix_lbl = String::from_utf8_lossy(record[7].as_bytes()).to_string();
//     let status_cd = String::from_utf8_lossy(record[8].as_bytes()).to_string();
//     let voter_status_desc = String::from_utf8_lossy(record[9].as_bytes()).to_string();
//     let reason_cd = String::from_utf8_lossy(record[10].as_bytes()).to_string();
//     let voter_status_reason_desc =
//         String::from_utf8_lossy(record[11].as_bytes()).to_string();
//     let res_street_address = String::from_utf8_lossy(record[12].as_bytes()).to_string();
//     let res_city_desc = String::from_utf8_lossy(record[13].as_bytes()).to_string();
//     let state_cd = String::from_utf8_lossy(record[14].as_bytes()).to_string();
//     let zip_code = String::from_utf8_lossy(record[15].as_bytes()).to_string();
//     let mail_addr1 = String::from_utf8_lossy(record[16].as_bytes()).to_string();
//     let mail_addr2 = String::from_utf8_lossy(record[17].as_bytes()).to_string();
//     let mail_addr3 = String::from_utf8_lossy(record[18].as_bytes()).to_string();
//     let mail_addr4 = String::from_utf8_lossy(record[19].as_bytes()).to_string();
//     let mail_city = String::from_utf8_lossy(record[20].as_bytes()).to_string();
//     let mail_state = String::from_utf8_lossy(record[21].as_bytes()).to_string();
//     let mail_zipcode = String::from_utf8_lossy(record[22].as_bytes()).to_string();
//     let full_phone_number = String::from_utf8_lossy(record[23].as_bytes()).to_string();
//     let confidential_ind = String::from_utf8_lossy(record[24].as_bytes()).to_string();
//     let registr_dt = String::from_utf8_lossy(record[25].as_bytes()).to_string();
//     let race_code = String::from_utf8_lossy(record[26].as_bytes()).to_string();
//     let ethnic_code = String::from_utf8_lossy(record[27].as_bytes()).to_string();
//     let party_cd = String::from_utf8_lossy(record[28].as_bytes()).to_string();
//     let gender_code = String::from_utf8_lossy(record[29].as_bytes()).to_string();
//     let age_at_year_end = String::from_utf8_lossy(record[31].as_bytes()).to_string();
//     let birth_state = String::from_utf8_lossy(record[32].as_bytes()).to_string();
//     let drivers_lic = String::from_utf8_lossy(record[33].as_bytes()).to_string();
//     let precinct_abbrv = String::from_utf8_lossy(record[34].as_bytes()).to_string();
//     let precinct_desc = String::from_utf8_lossy(record[35].as_bytes()).to_string();
//     let municipality_abbrv = String::from_utf8_lossy(record[36].as_bytes()).to_string();
//     let municipality_desc = String::from_utf8_lossy(record[37].as_bytes()).to_string();
//     let ward_abbrv = String::from_utf8_lossy(record[38].as_bytes()).to_string();
//     let ward_desc = String::from_utf8_lossy(record[39].as_bytes()).to_string();
//     let cong_dist_abbrv = String::from_utf8_lossy(record[40].as_bytes()).to_string();
//     let super_court_abbrv = String::from_utf8_lossy(record[41].as_bytes()).to_string();
//     let judic_dist_abbrv = String::from_utf8_lossy(record[42].as_bytes()).to_string();
//     let nc_senate_abbrv = String::from_utf8_lossy(record[43].as_bytes()).to_string();
//     let nc_house_abbrv = String::from_utf8_lossy(record[44].as_bytes()).to_string();
//     let county_commiss_abbrv = String::from_utf8_lossy(record[45].as_bytes()).to_string();
//     let county_commiss_desc = String::from_utf8_lossy(record[46].as_bytes()).to_string();
//     let township_abbrv = String::from_utf8_lossy(record[47].as_bytes()).to_string();
//     let township_desc = String::from_utf8_lossy(record[48].as_bytes()).to_string();
//     let school_dist_abbrv = String::from_utf8_lossy(record[49].as_bytes()).to_string();
//     let school_dist_desc = String::from_utf8_lossy(record[50].as_bytes()).to_string();
//     let fire_dist_abbrv = String::from_utf8_lossy(record[51].as_bytes()).to_string();
//     let fire_dist_desc = String::from_utf8_lossy(record[52].as_bytes()).to_string();
//     let water_dist_abbrv = String::from_utf8_lossy(record[53].as_bytes()).to_string();
//     let water_dist_desc = String::from_utf8_lossy(record[54].as_bytes()).to_string();
//     let sewer_dist_abbrv = String::from_utf8_lossy(record[55].as_bytes()).to_string();
//     let sewer_dist_desc = String::from_utf8_lossy(record[56].as_bytes()).to_string();
//     let sanit_dist_abbrv = String::from_utf8_lossy(record[57].as_bytes()).to_string();
//     let sanit_dist_desc = String::from_utf8_lossy(record[58].as_bytes()).to_string();
//     let rescue_dist_abbrv = String::from_utf8_lossy(record[59].as_bytes()).to_string();
//     let rescue_dist_desc = String::from_utf8_lossy(record[60].as_bytes()).to_string();
//     let munic_dist_abbrv = String::from_utf8_lossy(record[61].as_bytes()).to_string();
//     let munic_dist_desc = String::from_utf8_lossy(record[62].as_bytes()).to_string();
//     let dist_1_abbrv = String::from_utf8_lossy(record[63].as_bytes()).to_string();
//     let dist_1_desc = String::from_utf8_lossy(record[64].as_bytes()).to_string();
//     let vtd_abbrv = String::from_utf8_lossy(record[65].as_bytes()).to_string();
//     let vtd_description = String::from_utf8_lossy(record[66].as_bytes()).to_string();

//             conn.execute(
//                 &insert_query,
//                 params![
//                     county_id,
//                     county_desc,
//                     voter_reg_num,
//                     ncid,
//                     last_name,
//                     first_name,
//                     middle_name,
//                     name_suffix_lbl,
//                     status_cd,
//                     voter_status_desc,
//                     reason_cd,
//                     voter_status_reason_desc,
//                     res_street_address,
//                     res_city_desc,
//                     state_cd,
//                     zip_code,
//                     mail_addr1,
//                     mail_addr2,
//                     mail_addr3,
//                     mail_addr4,
//                     mail_city,
//                     mail_state,
//                     mail_zipcode,
//                     full_phone_number,
//                     confidential_ind,
//                     registr_dt,
//                     race_code,
//                     ethnic_code,
//                     party_cd,
//                     gender_code,
//                     birth_year,
//                     age_at_year_end,
//                     birth_state,
//                     drivers_lic,
//                     precinct_abbrv,
//                     precinct_desc,
//                     municipality_abbrv,
//                     municipality_desc,
//                     ward_abbrv,
//                     ward_desc,
//                     cong_dist_abbrv,
//                     super_court_abbrv,
//                     judic_dist_abbrv,
//                     nc_senate_abbrv,
//                     nc_house_abbrv,
//                     county_commiss_abbrv,
//                     county_commiss_desc,
//                     township_abbrv,
//                     township_desc,
//                     school_dist_abbrv,
//                     school_dist_desc,
//                     fire_dist_abbrv,
//                     fire_dist_desc,
//                     water_dist_abbrv,
//                     water_dist_desc,
//                     sewer_dist_abbrv,
//                     sewer_dist_desc,
//                     sanit_dist_abbrv,
//                     sanit_dist_desc,
//                     rescue_dist_abbrv,
//                     rescue_dist_desc,
//                     munic_dist_abbrv,
//                     munic_dist_desc,
//                     dist_1_abbrv,
//                     dist_1_desc,
//                     vtd_abbrv,
//                     vtd_description
//                 ],
//             )?;
//         } else {
//             eprintln!("Invalid data found in record: {:?}", record);
//         }
//     }

//     println!(
//         "Data was successfully loaded from '{}' into table '{}'.",
//         file_path, table_name
//     );
//     Ok(())
// }

//

// pub fn general_query(conn: &Connection, query: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
//     let mut stmt = conn.prepare(query)?;

//     // Get the column count before the query_map
//     let column_count = stmt.column_count();

//     let rows = stmt.query_map(params![], |row: &Row| {
//         let mut row_vec = Vec::new();
//         // Collect the column values as strings
//         for i in 0..column_count {
//             row_vec.push(row.get::<_, String>(i)?);
//         }
//         Ok(row_vec)
//     })?;

//     // Collect the results into a vector of vectors
//     let mut result = Vec::new();
//     for row in rows {
//         result.push(row?);
//     }

//     Ok(result)
// }

// pub fn general_query(conn: &Connection, query: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
//     let mut stmt = conn.prepare(query)?;

//     // Get the number of columns from the statement
//     let column_count = stmt.column_count();

//     let rows = stmt.query_map(params![], |row: &Row| {
//         let mut row_vec = Vec::new();

//         // Collect the column values as strings
//         for i in 0..column_count {
//             let value: String = row.get(i)?; // This will attempt to get the value as a String
//             row_vec.push(value);
//         }
//         Ok(row_vec)
//     })?;

//     // Collect the results into a vector of vectors
//     let result = Vec::new();
//     for row in rows {
//         let row = row?;
//         println!("{:?}", row);
//     }

//     Ok(result)
// }
