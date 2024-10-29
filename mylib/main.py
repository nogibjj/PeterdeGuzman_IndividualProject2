from lib import load_pollingplaces


if __name__ == "__main__":

    load_pollingplaces(
        dataset=f"{main_directory}/data/pollingplaces_2020.csv", year=2020
    )
