# parquet-tools

Show, inspect, and manipulate Parquet file.

## Usage
- `parquet-tools cat path/to/parquet` : display the content of a Parquet file
- `parquet-tools head path/to/parquet` : display the first few lines of a Parquet file
- `parquet-tools schema path/to/parquet` : print the schema of a Parquet file
- `parquet-tools meta path/to/parquet` : print the metadata of a Parquet file
- `parquet-tools dump path/to/parquet` : print the content and metadata of a Parquet file
- `parquet-tools merge path/to/parquet1 path/to/parquet2 path/to/target_parquet` : concatenate several Parquet files into the target one
- `parquet-tools rowcount path/to/parquet` : print the count of rows in a Parquet file
- `parquet-tools column-index path/to/parquet` : print the column and offset indexes of a Parquet file

## Categories
files, documents, text

## Links
- More information: https://github.com/apache/parquet-java
- tldr page: https://tldr.inbrowser.app/pages/common/parquet-tools
