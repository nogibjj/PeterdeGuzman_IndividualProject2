import os


def test_query_log_exists():
    # Specify the file name
    file_name = "query_log.md"

    # Check if the file exists
    if os.path.isfile(file_name):
        print(f"The file '{file_name}' exists.")
        assert True
    else:
        print(f"The file '{file_name}' does not exist.")
        return False


def test_queryresult():
    file_name = "query_log.md"
    search_string = (
        "SELECT COUNT(DISTINCT county_name) AS distinct_count FROM pollingplaces_2020"
    )
    try:
        with open(file_name, "r") as file:
            content = file.read()
            if search_string in content:
                print(f"The string '{search_string}' is found in '{file_name}'.")
                assert True
            else:
                print(f"The string '{search_string}' is not found in '{file_name}'.")
                return False
    except FileNotFoundError:
        print(f"The file '{file_name}' does not exist.")
        return False
