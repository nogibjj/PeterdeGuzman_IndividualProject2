from mylib.lib import load_pollingplaces, general_query
from test_main import test_query_log_exists, test_queryresult

# Logic to run all ETL-Query functions with SQLite for polling place data


if __name__ == "__main__":
    load_pollingplaces(
        dataset="/Users/pdeguz01/Documents/git/PeterdeGuzman_IndividualProject2/votesqlite/data/polling_place_20201103.csv",
        year=2020,
    )
    general_query(
        "SELECT COUNT(DISTINCT county_name) AS distinct_count FROM pollingplaces_2020"
    )
    test_query_log_exists()
    file_name = "query_log.md"
    search_string = (
        "SELECT COUNT(DISTINCT county_name) AS distinct_count FROM pollingplaces_2020"
    )
    test_queryresult(file_name, search_string)
