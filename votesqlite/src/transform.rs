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

    let insert_query = format!(
        "Insert INTO {} (county_id,
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
        vtd_desc) (?, ?, ?)",
        table_name
    );
    // for loop to set schema
    for result in rdr.records() {
        let record = result?;
        let county_id: i32 = record[0].parse()?;
        let county_desc: &str = &record[1];
        let voter_reg_num: &str = &record[2];
        let ncid: &str = &record[3];
        let last_name: &str = &record[4];
        let first_name: &str = &record[5];
        let middle_name: &str = &record[6];
        let name_suffix_lbl: &str = &record[7];
        let status_cd: &str = &record[8];
        let voter_status_desc: &str = &record[9];
        let reason_cd: &str = &record[10];
        let voter_status_reason_desc: &str = &record[11];
        let res_street_address: &str = &record[12];
        let res_city_desc: &str = &record[13];
        let state_cd: &str = &record[14];
        let zip_code: &str = &record[15];
        let mail_addr1: &str = &record[16];
        let mail_addr2: &str = &record[17];
        let mail_addr3: &str = &record[18];
        let mail_addr4: &str = &record[19];
        let mail_city: &str = &record[20];
        let mail_state: &str = &record[21];
        let mail_zipcode: &str = &record[22];
        let full_phone_number: &str = &record[23];
        let confidential_ind: &str = &record[24];
        let registr_dt: &str = &record[25];
        let race_code: &str = &record[26];
        let ethnic_code: &str = &record[27];
        let party_cd: &str = &record[28];
        let gender_code: &str = &record[29];
        let birth_year: i32 = record[30].parse()?;
        let age_at_year_end: &str = &record[31];
        let birth_state: &str = &record[32];
        let drivers_lic: &str = &record[33];
        let precinct_abbrv: &str = &record[34];
        let precinct_desc: &str = &record[35];
        let municipality_abbrv: &str = &record[36];
        let municipality_desc: &str = &record[37];
        let ward_abbrv: &str = &record[38];
        let ward_desc: &str = &record[39];
        let cong_dist_abbrv: &str = &record[40];
        let super_court_abbrv: &str = &record[41];
        let judic_dist_abbrv: &str = &record[42];
        let nc_senate_abbrv: &str = &record[43];
        let nc_house_abbrv: &str = &record[44];
        let county_commiss_abbrv: &str = &record[45];
        let county_commiss_desc: &str = &record[46];
        let township_abbrv: &str = &record[47];
        let township_desc: &str = &record[48];
        let school_dist_abbrv: &str = &record[49];
        let school_dist_desc: &str = &record[50];
        let fire_dist_abbrv: &str = &record[51];
        let fire_dist_desc: &str = &record[52];
        let water_dist_abbrv: &str = &record[53];
        let water_dist_desc: &str = &record[54];
        let sewer_dist_abbrv: &str = &record[55];
        let sewer_dist_desc: &str = &record[56];
        let sanit_dist_abbrv: &str = &record[57];
        let sanit_dist_desc: &str = &record[58];
        let rescue_dist_abbrv: &str = &record[59];
        let rescue_dist_desc: &str = &record[60];
        let munic_dist_abbrv: &str = &record[61];
        let munic_dist_desc: &str = &record[62];
        let dist_1_abbrv: &str = &record[63];
        let dist_1_desc: &str = &record[64];
        let vtd_abbrv: &str = &record[65];
        let vtd_description: &str = &record[66];

        conn.execute(
            &insert_query,
            params![
                county_id,
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
                vtd_description
            ],
        )?;
    }

    println!(
        "Data was successfully loaded from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

//CRUD Operations

//Create table with the new schema
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (county_id INTEGER,
                county_desc TEXT,
                voter_reg_num TEXT,
                ncid TEXT,
                last_name TEXT,
                first_name TEXT,
                middle_name TEXT,
                name_suffix_lbl TEXT,
                status_cd TEXT,
                voter_status_desc TEXT,
                reason_cd TEXT,
                voter_status_reason_desc TEXT,
                res_street_address TEXT,
                res_city_desc TEXT,
                state_cd TEXT,
                zip_code TEXT,
                mail_addr1 TEXT,
                mail_addr2 TEXT,
                mail_addr3 TEXT,
                mail_addr4 TEXT,
                mail_city TEXT,
                mail_state TEXT,
                mail_zipcode TEXT,
                full_phone_number TEXT,
                confidential_ind TEXT,
                registr_dt TEXT,
                race_code TEXT,
                ethnic_code TEXT,
                party_cd TEXT,
                gender_code TEXT,
                birth_year INTEGER,
                age_at_year_end TEXT,
                birth_state TEXT,
                drivers_lic TEXT,
                precinct_abbrv TEXT,
                precinct_desc TEXT,
                municipality_abbrv TEXT,
                municipality_desc TEXT,
                ward_abbrv TEXT,
                ward_desc TEXT,
                cong_dist_abbrv TEXT,
                super_court_abbrv TEXT,
                judic_dist_abbrv TEXT,
                nc_senate_abbrv TEXT,
                nc_house_abbrv TEXT,
                county_commiss_abbrv TEXT,
                county_commiss_desc TEXT,
                township_abbrv TEXT,
                township_desc TEXT,
                school_dist_abbrv TEXT,
                school_dist_desc TEXT,
                fire_dist_abbrv TEXT,
                fire_dist_desc TEXT,
                water_dist_abbrv TEXT,
                water_dist_desc TEXT,
                sewer_dist_abbrv TEXT,
                sewer_dist_desc TEXT,
                sanit_dist_abbrv TEXT,
                sanit_dist_desc TEXT,
                rescue_dist_abbrv TEXT,
                rescue_dist_desc TEXT,
                munic_dist_abbrv TEXT,
                munic_dist_desc TEXT,
                dist_1_abbrv TEXT,
                dist_1_desc TEXT,
                vtd_abbrv TEXT,
                vtd_description TEXT
    )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing, but will return error if one occurs
}

//Read data from the table
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    //Prepare query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    //Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        //Using the specific schema
        let county_id: i32 = row.get(0)?;
        let county_desc: String = row.get(1)?;
        let voter_reg_num: String = row.get(2)?;
        let ncid: String = row.get(3)?;
        let last_name: String = row.get(4)?;
        let first_name: String = row.get(5)?;
        let middle_name: String = row.get(6)?;
        let name_suffix_lbl: String = row.get(7)?;
        let status_cd: String = row.get(8)?;
        let voter_status_desc: String = row.get(9)?;
        let reason_cd: String = row.get(10)?;
        let voter_status_reason_desc: String = row.get(11)?;
        let res_street_address: String = row.get(12)?;
        let res_city_desc: String = row.get(13)?;
        let state_cd: String = row.get(14)?;
        let zip_code: String = row.get(15)?;
        let mail_addr1: String = row.get(16)?;
        let mail_addr2: String = row.get(17)?;
        let mail_addr3: String = row.get(18)?;
        let mail_addr4: String = row.get(19)?;
        let mail_city: String = row.get(20)?;
        let mail_state: String = row.get(21)?;
        let mail_zipcode: String = row.get(22)?;
        let full_phone_number: String = row.get(23)?;
        let confidential_ind: String = row.get(24)?;
        let registr_dt: String = row.get(25)?;
        let race_code: String = row.get(26)?;
        let ethnic_code: String = row.get(27)?;
        let party_cd: String = row.get(28)?;
        let gender_code: String = row.get(29)?;
        let birth_year: i32 = row.get(30)?;
        let age_at_year_end: String = row.get(31)?;
        let birth_state: String = row.get(32)?;
        let drivers_lic: String = row.get(33)?;
        let precinct_abbrv: String = row.get(34)?;
        let precinct_desc: String = row.get(35)?;
        let municipality_abbrv: String = row.get(36)?;
        let municipality_desc: String = row.get(37)?;
        let ward_abbrv: String = row.get(38)?;
        let ward_desc: String = row.get(39)?;
        let cong_dist_abbrv: String = row.get(40)?;
        let super_court_abbrv: String = row.get(41)?;
        let judic_dist_abbrv: String = row.get(42)?;
        let nc_senate_abbrv: String = row.get(43)?;
        let nc_house_abbrv: String = row.get(44)?;
        let county_commiss_abbrv: String = row.get(45)?;
        let county_commiss_desc: String = row.get(46)?;
        let township_abbrv: String = row.get(47)?;
        let township_desc: String = row.get(48)?;
        let school_dist_abbrv: String = row.get(49)?;
        let school_dist_desc: String = row.get(50)?;
        let fire_dist_abbrv: String = row.get(51)?;
        let fire_dist_desc: String = row.get(52)?;
        let water_dist_abbrv: String = row.get(53)?;
        let water_dist_desc: String = row.get(54)?;
        let sewer_dist_abbrv: String = row.get(55)?;
        let sewer_dist_desc: String = row.get(56)?;
        let sanit_dist_abbrv: String = row.get(57)?;
        let sanit_dist_desc: String = row.get(58)?;
        let rescue_dist_abbrv: String = row.get(59)?;
        let rescue_dist_desc: String = row.get(60)?;
        let munic_dist_abbrv: String = row.get(61)?;
        let munic_dist_desc: String = row.get(62)?;
        let dist_1_abbrv: String = row.get(63)?;
        let dist_1_desc: String = row.get(64)?;
        let vtd_abbrv: String = row.get(65)?;
        let vtd_description: String = row.get(66)?;
        Ok((
            county_id,
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
            vtd_description,
        ))
    })?;
    //Iterate over the rows and print the results
    for row in rows {
        let (
            county_id,
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
            vtd_description,
        ) = row?;
        println!(
            "county_id: {},
                county_desc: {},
                voter_reg_num: {},
                ncid: {},
                last_name: {},
                first_name: {},
                middle_name: {},
                name_suffix_lbl: {},
                status_cd: {},
                voter_status_desc: {},
                reason_cd: {},
                voter_status_reason_desc: {},
                res_street_address: {},
                res_city_desc: {},
                state_cd: {},
                zip_code: {},
                mail_addr1: {},
                mail_addr2: {},
                mail_addr3: {},
                mail_addr4: {},
                mail_city: {},
                mail_state: {},
                mail_zipcode: {},
                full_phone_number: {},
                confidential_ind: {},
                registr_dt: {},
                race_code: {},
                ethnic_code: {},
                party_cd: {},
                gender_code: {},
                birth_year: {},
                age_at_year_end: {},
                birth_state: {},
                drivers_lic: {},
                precinct_abbrv: {},
                precinct_desc: {},
                municipality_abbrv: {},
                municipality_desc: {},
                ward_abbrv: {},
                ward_desc: {},
                cong_dist_abbrv: {},
                super_court_abbrv: {},
                judic_dist_abbrv: {},
                nc_senate_abbrv: {},
                nc_house_abbrv: {},
                county_commiss_abbrv: {},
                county_commiss_desc: {},
                township_abbrv: {},
                township_desc: {},
                school_dist_abbrv: {},
                school_dist_desc: {},
                fire_dist_abbrv: {},
                fire_dist_desc: {},
                water_dist_abbrv: {},
                water_dist_desc: {},
                sewer_dist_abbrv: {},
                sewer_dist_desc: {},
                sanit_dist_abbrv: {},
                sanit_dist_desc: {},
                rescue_dist_abbrv: {},
                rescue_dist_desc: {},
                munic_dist_abbrv: {},
                munic_dist_desc: {},
                dist_1_abbrv: {},
                dist_1_desc: {},
                vtd_abbrv: {},
                vtd_description: {}",
            county_id,
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
            vtd_description
        );
    }
    Ok(())
}

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
