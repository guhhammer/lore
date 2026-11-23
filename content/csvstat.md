# csvstat

Print descriptive statistics for all columns in a CSV file. Included in csvkit.

## Usage
- `csvstat data.csv` : show all stats for all columns
- `csvstat -c 2,4 data.csv` : show all stats for columns 2 and 4
- `csvstat --sum data.csv` : show sums for all columns
- `csvstat -c 3 --len data.csv` : show the max value length for column 3
- `csvstat -c name --unique data.csv` : show the number of unique values in the "name" column

## Categories
text, documents, files

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvstat.html
- tldr page: https://tldr.inbrowser.app/pages/common/csvstat
