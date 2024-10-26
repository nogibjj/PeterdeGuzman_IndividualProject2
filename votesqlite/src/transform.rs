use csv::ReaderBuilder;
use csv::WriterBuilder;
use encoding::all::WINDOWS_1252;
use encoding::{EncoderTrap, Encoding};
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

//Transform and Load Functions

//Future expansion:

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

pub fn transform_voterreg(
    txtfile: &str,
    county: &str,
    date: &str,
    directory: &str,
) -> Result<(), Box<dyn Error>> {
    // Open the input file
    let file = File::open(txtfile)?;
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    // Check the expected columns
    let expected_columns = vec![
        "county_id",
        "county_desc",
        "voter_reg_num",
        "ncid",
        "last_name",
        "first_name",
        "middle_name",
        "name_suffix_lbl",
        "status_cd",
        "voter_status_desc",
        "reason_cd",
        "voter_status_reason_desc",
        "res_street_address",
        "res_city_desc",
        "state_cd",
        "zip_code",
        "mail_addr1",
        "mail_addr2",
        "mail_addr3",
        "mail_addr4",
        "mail_city",
        "mail_state",
        "mail_zipcode",
        "full_phone_number",
        "confidential_ind",
        "registr_dt",
        "race_code",
        "ethnic_code",
        "party_cd",
        "gender_code",
        "birth_year",
        "age_at_year_end",
        "birth_state",
        "drivers_lic",
        "precinct_abbrv",
        "precinct_desc",
        "municipality_abbrv",
        "municipality_desc",
        "ward_abbrv",
        "ward_desc",
        "cong_dist_abbrv",
        "super_court_abbrv",
        "judic_dist_abbrv",
        "nc_senate_abbrv",
        "nc_house_abbrv",
        "county_commiss_abbrv",
        "county_commiss_desc",
        "township_abbrv",
        "township_desc",
        "school_dist_abbrv",
        "school_dist_desc",
        "fire_dist_abbrv",
        "fire_dist_desc",
        "water_dist_abbrv",
        "water_dist_desc",
        "sewer_dist_abbrv",
        "sewer_dist_desc",
        "sanit_dist_abbrv",
        "sanit_dist_desc",
        "rescue_dist_abbrv",
        "rescue_dist_desc",
        "munic_dist_abbrv",
        "munic_dist_desc",
        "dist_1_abbrv",
        "dist_1_desc",
        "vtd_abbrv",
        "vtd_desc",
    ];

    // Check if the columns match
    let headers = reader.headers()?;
    let actual_columns: Vec<&str> = headers.iter().collect();

    if actual_columns.len() == 67
        && expected_columns
            .iter()
            .all(|col| actual_columns.contains(col))
    {
        println!("All expected columns are present in dataframe.");
    } else {
        println!("The CSV does not have the correct columns.");
        println!("Expected columns: {:?}", expected_columns);
        println!("Actual columns: {:?}", actual_columns);
    }

    // Prepare the output file path
    let filepath = Path::new(directory).join(format!("voterreg_{}{}.csv", county, date));
    let output_file = File::create(&filepath)?;
    let mut writer = WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(BufWriter::new(output_file));

    // Write the headers to the output file
    writer.write_record(&actual_columns)?;

    // Read and convert records, then write to the output file
    for result in reader.records() {
        let record = result?;

        // Try to convert each field from Windows-1252 to UTF-8
        let utf16_record: Vec<String> = record
            .iter()
            .filter_map(|field| {
                // Encode to bytes using Windows-1252
                let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();
                // Attempt to decode bytes to String
                String::from_utf8(bytes)
                    .map_err(|err| {
                        eprintln!("Error decoding field: {:?}", err);
                        // Return None for invalid UTF-8
                    })
                    .ok() // Convert Result to Option
            })
            .collect();

        // Check if we have a full record of valid fields
        if utf16_record.len() == record.len() {
            // Only write valid records to the output file
            writer.write_record(&utf16_record)?;
        } else {
            // Skip this record if there was any decoding error
            eprintln!("Skipping record due to invalid UTF-8 bytes.");
        }
    }

    writer.flush()?;
    println!("Data successfully transformed and saved to {:?}", filepath);

    Ok(())
}

//Load voter registration dataset

pub fn load_voterreg(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!("Insert INTO {} (county_id,
        county_desc,
        voter_reg_num,
        ncid,
        last_name,
        first_name,
        middle_name,
        name_suffix_lbl,
        status_cd,
        voter_status_desc,
        reason_cd,
        voter_status_reason_desc,
        res_street_address,
        res_city_desc,
        state_cd,
        zip_code,
        mail_addr1,
        mail_addr2,
        mail_addr3,
        mail_addr4,
        mail_city,
        mail_state,
        mail_zipcode,
        full_phone_number,
        confidential_ind,
        registr_dt,
        race_code,
        ethnic_code,
        party_cd,
        gender_code,
        birth_year,
        age_at_year_end,
        birth_state,
        drivers_lic,
        precinct_abbrv,
        precinct_desc,
        municipality_abbrv,
        municipality_desc,
        ward_abbrv,
        ward_desc,
        cong_dist_abbrv,
        super_court_abbrv,
        judic_dist_abbrv,
        nc_senate_abbrv,
        nc_house_abbrv,
        county_commiss_abbrv,
        county_commiss_desc,
        township_abbrv,
        township_desc,
        school_dist_abbrv,
        school_dist_desc,
        fire_dist_abbrv,
        fire_dist_desc,
        water_dist_abbrv,
        water_dist_desc,
        sewer_dist_abbrv,
        sewer_dist_desc,
        sanit_dist_abbrv,
        sanit_dist_desc,
        rescue_dist_abbrv,
        rescue_dist_desc,
        munic_dist_abbrv,
        munic_dist_desc,
        dist_1_abbrv,
        dist_1_desc,
        vtd_abbrv,
        vtd_desc) (?, ?, ?)", table_name);
    // for loop to set schema
    for result in rdr.records() {
        let record = result?;
        let id: i32 = record[0].parse()?;
        let name: &str = &record[1];
        let age: i32 = record[2].parse()?;
    }

    println!(
        "Data was successfully loaded from '{}' into table '{}'.",
        file_path, table_name);
    Ok(())
}
