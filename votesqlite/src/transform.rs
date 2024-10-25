use csv::ReaderBuilder;
use csv::WriterBuilder;
use encoding::all::WINDOWS_1252;
use encoding::{EncoderTrap, Encoding};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

//Transform and Load Functions

//Future expansion:

//Transform Windows-1521 encoded files to UTF-16
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

        // Convert each field from Windows-1252 to UTF-8 and write to the new file
        let utf16_record: Vec<String> = record
            .iter()
            .map(|field| {
                // Encode to bytes using Windows-1252
                let bytes = WINDOWS_1252.encode(field, EncoderTrap::Strict).unwrap();

                // Decode bytes to String with error handling
                String::from_utf8(bytes).unwrap_or_else(|err| {
                    eprintln!("Error decoding field: {:?}", err);
                    // Return a placeholder for invalid UTF-8 bytes
                    String::from("INVALID_UTF8")
                })
            })
            .collect();

        writer.write_record(&utf16_record)?;
    }

    writer.flush()?;
    println!("Data successfully transformed and saved to {:?}", filepath);

    Ok(())
}

//Load voter registration dataset
