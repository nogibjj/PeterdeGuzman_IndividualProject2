import csv
import sqlite3
import time
from memory_profiler import memory_usage


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


# Define a global variable for the log file
LOG_FILE = "query_log.md"


def log_query(query, result="none"):
    """Logs the executed SQL query and its result to a markdown file."""
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")
        file.write(f"```response from sqlite\n{result}\n```\n\n")


# CRUD Operations


def general_query(query, params=None):
    conn = sqlite3.connect("pollingplaces_2020.db")
    cursor = conn.cursor()
    # execute read
    start_time = time.time()
    mem_usage_before = memory_usage(max_usage=True)
    try:
        if query.strip().upper().startswith("SELECT"):
            cursor.execute(query, params or [])
            results = cursor.fetchall()
            execution_time = time.time() - start_time
            mem_usage_after = memory_usage(max_usage=True)
            log_query(
                query,
                {
                    "results": results,
                    "execution_time": execution_time,
                    "memory_usage_before": mem_usage_before,
                    "memory_usage": mem_usage_after,
                },
            )
            return results
        else:
            cursor.execute(query, params or [])
            conn.commit()
            execution_time = time.time() - start_time
            mem_usage_after = memory_usage(max_usage=True)
            log_query(
                query,
                {
                    "message": "Query executed successfully",
                    "execution_time": execution_time,
                    "memory_usage_before": mem_usage_before,
                    "memory_usage": mem_usage_after,
                },
            )
            return "Query executed successfully"
    except sqlite3.Error as e:
        execution_time = time.time() - start_time
        mem_usage_after = memory_usage(max_usage=True)
        log_query(
            query,
            {
                "error": str(e),
                "execution_time": execution_time,
                "memory_usage_before": mem_usage_before,
                "memory_usage": mem_usage_after,
            },
        )
        return f"An error occurred: {e}"
    finally:
        conn.close()
