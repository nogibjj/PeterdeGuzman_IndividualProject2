from lib import load_pollingplaces

# Logic to run all ETL-Query functions with SQLite for polling place data


if __name__ == "__main__":

    load_pollingplaces(
        dataset="/Users/pdeguz01/Documents/git/PeterdeGuzman_IndividualProject2/votesqlite/data/polling_place_20201103.csv",
        year=2020,
    )
