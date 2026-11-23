# sqlfluff

Lint and auto-format SQL files across multiple dialects.

## Usage
- `sqlfluff lint -d dialect path/to/file_or_directory` : lint an SQL file or directory with a specific dialect
- `sqlfluff format -d dialect path/to/file_or_directory` : auto-format an SQL file or directory with a specific dialect
- `sqlfluff fix -d dialect -r rule1,rule2,... path/to/file_or_directory` : automatically fix linting violations in an SQL file or directory
- `sqlfluff parse -d dialect path/to/file_or_directory` : parse an SQL file or directory and display the parse tree
- `sqlfluff dialects` : show all supported SQL dialects

## Categories
files, databases

## Links
- More information: https://docs.sqlfluff.com/en/stable/reference/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/sqlfluff
