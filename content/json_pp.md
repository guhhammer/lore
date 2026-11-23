# json_pp

Pretty-prints JSON, reading from a file or standard input.

## Usage
- `json_pp < data.json` : pretty-print a JSON file
- `echo '{"a":1}' | json_pp` : pretty-print JSON from a pipe
- `json_pp -json_opt canonical < data.json` : sort the keys for a stable output

## Categories
text, files
