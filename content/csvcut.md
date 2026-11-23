# csvcut

Filter and truncate CSV files. Like Unix's `cut` command, but for tabular data. Included in csvkit.

## Usage
- `csvcut -n data.csv` : print indices and names of all columns
- `csvcut -c 1,3 data.csv` : extract the first and third columns
- `csvcut -C 4 data.csv` : extract all columns except the fourth one
- `csvcut -c id,"first name" data.csv` : extract the columns named "id" and "first name" (in that order)

## Categories
text, archives

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvcut.html
- tldr page: https://tldr.inbrowser.app/pages/common/csvcut
