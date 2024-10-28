"""
Transform and load with SQlite3 database
"""

from databricks import sql
import csv
import pandas as pd
import os
from dotenv import load_dotenv

csv.field_size_limit(100000000)

# set cwd
# main_directory = "/Users/pdeguz01/Documents/git/PeterdeGuzman_Mini6/"
# os.chdir(main_directory)

# Transform functions
# might need to create two different transforms
# voter reg encoding is Windows-1252
# vote history encoding is ascii


def transform_voterreg(txtfile, county, date, directory):
    # Transforming voter reg data from encoding="Windows-1252" to encoding="utf-16"
    df = pd.read_csv(
        txtfile,
        delimiter="\t",
        encoding="windows-1252",
        on_bad_lines="skip",
        low_memory=False,
    )
    # checking the expected columns
    expected_columns = [
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
    ]
    if len(df.columns) == 67 and all(col in expected_columns for col in df.columns):
        print("All expected columns are present in dataframe.")
    else:
        print("The CSV does not have the correct columns.")
        print("Expected columns:", expected_columns)
        print("Actual columns:", df.columns.tolist())
    # Save as UTF-16 encoded CSV
    filepath = os.path.join(directory, f"voterreg_{county}{date}.csv")

    df.to_csv(
        filepath,
        sep="\t",
        encoding="utf-16",
        index=False,
        quoting=csv.QUOTE_MINIMAL,
    )


def load_voterreg(dataset):
    """Loads data into the databricks database"""
    payload = csv.reader(
        open(dataset, encoding="utf-16"),
        delimiter="\t",
    )
    next(payload)
    load_dotenv()
    server_h = os.getenv("sql_server_host")
    access_token = os.getenv("databricks_api_key")
    http_path = os.getenv("sql_http")
    with sql.connect(
        server_hostname=server_h,
        http_path=http_path,
        access_token=access_token,
    ) as conn:
        with conn.cursor() as c:
            # generate new table for the database
            c.execute(
                """CREATE TABLE IF NOT EXISTS ped19_voterreg (
                county_id INT,
                county_desc STRING,
                voter_reg_num STRING,
                ncid STRING,
                last_name STRING,
                first_name STRING,
                middle_name STRING,
                name_suffix_lbl STRING,
                status_cd STRING,
                voter_status_desc STRING,
                reason_cd STRING,
                voter_status_reason_desc STRING,
                res_street_address STRING,
                res_city_desc STRING,
                state_cd STRING,
                zip_code STRING,
                mail_addr1 STRING,
                mail_addr2 STRING,
                mail_addr3 STRING,
                mail_addr4 STRING,
                mail_city STRING,
                mail_state STRING,
                mail_zipcode STRING,
                full_phone_number STRING,
                confidential_ind STRING,
                registr_dt STRING,
                race_code STRING,
                ethnic_code STRING,
                party_cd STRING,
                gender_code STRING,
                birth_year INT,
                age_at_year_end STRING,
                birth_state STRING,
                drivers_lic STRING,
                precinct_abbrv STRING,
                precinct_desc STRING,
                municipality_abbrv STRING,
                municipality_desc STRING,
                ward_abbrv STRING,
                ward_desc STRING,
                cong_dist_abbrv STRING,
                super_court_abbrv STRING,
                judic_dist_abbrv STRING,
                nc_senate_abbrv STRING,
                nc_house_abbrv STRING,
                county_commiss_abbrv STRING,
                county_commiss_desc STRING,
                township_abbrv STRING,
                township_desc STRING,
                school_dist_abbrv STRING,
                school_dist_desc STRING,
                fire_dist_abbrv STRING,
                fire_dist_desc STRING,
                water_dist_abbrv STRING,
                water_dist_desc STRING,
                sewer_dist_abbrv STRING,
                sewer_dist_desc STRING,
                sanit_dist_abbrv STRING,
                sanit_dist_desc STRING,
                rescue_dist_abbrv STRING,
                rescue_dist_desc STRING,
                munic_dist_abbrv STRING,
                munic_dist_desc STRING,
                dist_1_abbrv STRING,
                dist_1_desc STRING,
                vtd_abbrv STRING,
                vtd_description STRING);"""
            )
            # insert
            string_sql = "INSERT into ped19_voterreg VALUES"
            for i in payload:
                string_sql += "\n" + str(tuple(i)) + ","
            string_sql = string_sql[:-1] + ";"
            # print(string_sql[:-1])
            c.execute(string_sql)
        c.close()
        conn.close()
    print("Successfully loaded data to Databricks")
    return "success"


# Define a global variable for the log file
LOG_FILE = "query_log.md"


def log_query(query, result="none"):
    """adds to a query markdown file"""
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")
        file.write(f"```response from databricks\n{result}\n```\n\n")


def general_query(query):
    """runs a query a user inputs"""

    load_dotenv()
    server_h = os.getenv("sql_server_host")
    access_token = os.getenv("databricks_api_key")
    http_path = os.getenv("sql_http")
    with sql.connect(
        server_hostname=server_h,
        http_path=http_path,
        access_token=access_token,
    ) as conn:
        c = conn.cursor()
        c.execute(query)
        result = c.fetchall()
    c.close()
    log_query(f"{query}", result)


if __name__ == "__main__":
    transform_voterreg(
        txtfile="/Users/pdeguz01/Documents/git/PeterdeGuzman_IndividualProject2/votesqlite/data/ncvoter32.txt",
        county="Durham",
        date="241028",
        directory="data",
    )
    load_voterreg(
        netid="ped19",
        dataset="/Users/pdeguz01/Documents/git/PeterdeGuzman_IndividualProject2/votesqlite/data/voterreg_Durham241028.csv",
        date="241028",
    )
