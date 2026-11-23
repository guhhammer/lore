# odpscmd tunnel

Data tunnel in ODPS (Open Data Processing Service).

## Usage
- `tunnel download table_name path/to/file;` : [Interactive] Download table to local file
- `tunnel upload path/to/file table_name/partition_spec;` : [Interactive] Upload local file to a table partition
- `tunnel upload path/to/file table_name -fd field_delim -rd record_delim;` : [Interactive] Upload table specifying field and record delimiters
- `tunnel upload path/to/file table_name -threads num;` : [Interactive] Upload table using multiple threads

## Categories
files, network, databases

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
- tldr page: https://tldr.inbrowser.app/pages/common/odpscmd-tunnel
