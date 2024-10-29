import csv
import sqlite3


def load_pollingplaces(dataset, year):
    data = open(dataset, newline="", encoding="utf-16")
    # NCSBE data includes null bytes, which must be removed
    payload = csv.reader((line.replace("\0", "") for line in data), delimiter="\t")
    db_name = "pollingplaces_"
    conn = sqlite3.connect(f"{db_name}{year}.db")
    c = conn.cursor()
    # generate new table for the database
    c.execute(f"DROP TABLE IF EXISTS {db_name}{year}")
    c.execute(
        f"""
            CREATE TABLE {db_name}{year} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            election_dt DATE,
            county_name TEXT,
            polling_place_id INTEGER,
            polling_place_name TEXT,
            precinct_name TEXT,
            house_num INTEGER,
            street_name TEXT,
            city TEXT,
            state TEXT,
            zip TEXT)
            """
    )
    # insert values
    c.executemany(
        f"""
            INSERT INTO {db_name}{year} (
            election_dt,
            county_name,
            polling_place_id,
            polling_place_name,
            precinct_name,
            house_num,
            street_name,
            city,
            state,
            zip)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
        payload,
    )
    conn.commit()
    conn.close()
    return f"{db_name}{year}.db"
