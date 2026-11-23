# gocr

Optical Character Recognition tool. Recognize characters using its engine, and prompt the user for unknown patterns to store them in a database.

## Usage
- `gocr -m 130 -p path/to/db_directory -i path/to/input_image.png -o path/to/output_file.txt` : recognize characters in the input image and output it in the given file. Put the database (p) in `path/to/db_directory` (verify that the folder exists or DB usage will silently be skipped). mode 130 means create + use + extend database
- `gocr -m 130 -p path/to/db_directory -i path/to/input_image.png -o path/to/output_file.txt -C "0..9"` : recognize characters and assume all Characters are numbers
- `gocr -m 130 -p path/to/db_directory -i path/to/input_image.png -o path/to/output_file.txt -a 100` : recognize characters with a certainty of 100% (characters have a higher chance to be treated as unknown)

## Categories
files, databases, shell

## Links
- More information: https://manned.org/gocr
- tldr page: https://tldr.inbrowser.app/pages/common/gocr
