# ffe

Extract fields from a flat database file and write to another format. A configuration file is required to interpret the input and format the output.

## Usage
- `ffe -c path/to/config.ffe path/to/input` : display all input data using the specified data configuration
- `ffe --output=path/to/output -c path/to/config.ffe path/to/input` : convert an input file to an output file in a new format
- `ffe -s structure -p format path/to/input` : select input structure and print format from definitions in `~/.fferc` configuration file
- `ffe -f "FirstName,LastName,Age" -c path/to/config.ffe path/to/input` : write only the selected fields
- `ffe -e "LastName=Smith" -c path/to/config.ffe path/to/input` : write only the records that match an expression
- `ffe -?` : display help

## Categories
files, archives, databases

## Links
- More information: https://ff-extractor.sourceforge.net/ffe.html
- tldr page: https://tldr.inbrowser.app/pages/common/ffe
