use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use csv::ReaderBuilder;
use sqlx::sqlite::SqlitePoolOptions;
use sql::{Connection, Executor};


// Creates Transform and Load functions

#[derive(Debug)]
struct PollingPlace {
    election_dt: String,
    county_name: String,
    polling_place_id: i32,
    polling_place_name: String,
    precinct_name: String,
    house_num: Option<i32>,
    street_name: String,
    city: String,
    state: String,
    zip: String,
}


//Load Polling Places data
pub fn load_pollingplaces(dataset: &str, year: &str) -> Result<String, Box<dyn Error>> {
    //Read CSV
    let file = File::open(dataset)?;
    
    //Connect to SQLite db

    //Create table

    //Insert values into db
}








//Future expansion:
//Transform Windows-1521 encoded files to UTF-16
//Load voter registration dataset

