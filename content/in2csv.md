# in2csv

Convert various tabular data formats to CSV. Included in csvkit.

## Usage
- `in2csv data.xls` : convert an XLS file to CSV
- `in2csv data.dbf >data.csv` : convert a DBF file to a CSV file
- `in2csv --sheet=sheet_name data.xlsx` : convert a specific sheet from an XLSX file to CSV
- `cat data.json | in2csv -f json >data.csv` : pipe a JSON file to in2csv

## Categories
text, files

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/in2csv.html
- tldr page: https://tldr.inbrowser.app/pages/common/in2csv
