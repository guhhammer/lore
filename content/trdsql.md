# trdsql

Execute SQL on CSV, LTSV, JSON, YAML, and TBLN files.

## Usage
- `trdsql -ocsv -oh "SELECT * FROM path/to/directory/*.json" | sed 's/\([^,]*\)/"&"/g' >path/to/file.csv` : convert object data from multiple JSON files to a CSV file with header (`-oh`) and double quote
- `trdsql "SELECT * FROM path/to/file.json::.list"` : interpret JSON list as a table and put objects inside as columns (`path/to/file.json: {"list":[{"age":"26","name":"Tanaka"}]}`)
- `trdsql -icsv -ih "SELECT column1,column2 FROM path/to/file*.csv WHERE column2 != '' ORDER BY column1 GROUP BY column1"` : manipulate complex SQL query with data from multiple CSV files with first line is header (`-ih`)
- `trdsql "SELECT column1,column2 FROM path/to/file1.csv UNION SELECT column1,column2 FROM path/to/file2.csv"` : merge content of 2 CSV files to one CSV file
- `trdsql -driver postgres -dsn "host=hostname port=5433 dbname=database_name" "SELECT 1"` : connect to PostgreSQL database
- `trdsql -driver mysql -dsn "username:password@hostname/database" -ih "CREATE TABLE table (column1 int, column2 varchar(20)) AS SELECT column3 AS column1,column2 FROM path/to/header_file.csv"` : create table data to MySQL database from CSV file
- `trdsql -iltsv "SELECT * FROM path/to/access.log.gz"` : show data from compress log files

## Categories
text, files, databases

## Links
- More information: https://noborus.github.io/trdsql/
- tldr page: https://tldr.inbrowser.app/pages/common/trdsql
