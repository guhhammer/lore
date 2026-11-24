# pytest

Run Python tests.

## Usage
- `pytest path/to/test_file1.py path/to/test_file2.py ...` : run tests from specific files
- `pytest -k expression` : run tests with names matching a specific keyword expression
- `pytest -x` : exit as soon as a test fails or encounters an error
- `pytest -m marker_name1 and not marker_name2` : run tests matching or excluding markers
- `pytest --sw` : run until a test failure, continuing from the last failing test
- `pytest -s` : run tests without capturing output
- `pytest -v` : run tests with increased verbosity, displaying individual test names

## Categories
development

## Links
- More information: https://docs.pytest.org/en/latest/how-to/usage.html
- tldr page: https://tldr.inbrowser.app/pages/common/pytest
- Wikipedia: https://en.wikipedia.org/wiki/Pytest
