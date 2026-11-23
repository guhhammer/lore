# xsv

A CSV toolkit written in Rust.

## Usage
- `xsv headers path/to/file.csv` : inspect the headers of a file
- `xsv count path/to/file.csv` : count the number of entries
- `xsv stats path/to/file.csv | xsv table` : get an overview of the shape of entries
- `xsv select column1,column2 path/to/file.csv` : select a few columns
- `xsv sample 10 path/to/file.csv` : show 10 random entries
- `xsv join --no-case column1 path/to/file1.csv column2 path/to/file2.csv | xsv table` : join a column from one file to another

## Categories
files, text

## Links
- More information: https://github.com/BurntSushi/xsv
- tldr page: https://tldr.inbrowser.app/pages/common/xsv
