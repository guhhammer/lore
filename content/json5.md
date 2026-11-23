# json5

Convert JSON5 files to JSON.

## Usage
- `echo input | json5` : convert JSON5 `stdin` to JSON `stdout`
- `json5 path/to/input_file.json5` : convert a JSON5 file to JSON and output to `stdout`
- `json5 path/to/input_file.json5 -o path/to/output_file.json` : convert a JSON5 file to the specified JSON file
- `json5 path/to/input_file.json5 -v` : validate a JSON5 file
- `json5 -s indent_amount` : specify the number of spaces to indent by (or "t" for tabs)
- `json5 -h` : display help

## Categories
files, text

## Links
- More information: https://json5.org/#cli
- tldr page: https://tldr.inbrowser.app/pages/common/json5
