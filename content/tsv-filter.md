# tsv-filter

Filter lines of a TSV file by running tests against individual fields.

## Usage
- `tsv-filter -H --eq field_name:number path/to/tsv_file` : print the lines where a specific column is numerically equal to a given number
- `tsv-filter --eq|ne|lt|le|gt|ge column_number:number path/to/tsv_file` : print the lines where a specific column is [eq]ual/non equal/less than/less than or equal/greater than/greater than or equal to a given number
- `tsv-filter --str-eq|ne|in-fld|not-in-fld column_number:string path/to/tsv_file` : print the lines where a specific column is [eq]ual/not equal/part of/not part of a given string
- `tsv-filter --not-empty column_number path/to/tsv_file` : filter for non-empty fields
- `tsv-filter --invert --not-empty column_number path/to/tsv_file` : print the lines where a specific column is empty
- `tsv-filter --eq column_number1:number --str-eq column_number2:string path/to/tsv_file` : print the lines that satisfy two conditions
- `tsv-filter --or --eq column_number1:number --str-eq column_number2:string path/to/tsv_file` : print the lines that match at least one condition
- `tsv-filter --count -H --eq field_name:number path/to/tsv_file` : count matching lines, interpreting first line as a Header

## Categories
text, files, documents

## Links
- More information: https://github.com/eBay/tsv-utils#tsv-filter
- tldr page: https://tldr.inbrowser.app/pages/common/tsv-filter
