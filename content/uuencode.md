# uuencode

Encode binary files into ASCII for transport via mediums that only support simple ASCII encoding.

## Usage
- `uuencode path/to/input_file output_file_name_after_decoding` : encode a file and print the result to `stdout`
- `uuencode -o path/to/output_file path/to/input_file output_file_name_after_decoding` : encode a file and write the result to a file
- `uuencode -m -o path/to/output_file path/to/input_file output_file_name_after_decoding` : encode a file using Base64 instead of the default uuencode encoding and write the result to a file

## Categories
files

## Links
- More information: https://manned.org/uuencode
- tldr page: https://tldr.inbrowser.app/pages/common/uuencode
