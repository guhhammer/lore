# uuidparse

Parse universally unique identifiers.

## Usage
- `uuidparse uuid1 uuid2 ...` : parse the specified UUIDs, use a tabular output format
- `command | uuidparse` : parse UUIDs from `stdin`
- `uuidparse -J uuid1 uuid2 ...` : use the JSON output format
- `uuidparse -n uuid1 uuid2 ...` : do not print a header line
- `uuidparse -r uuid1 uuid2 ...` : use the raw output format
- `uuidparse -o UUID,VARIANT,TYPE,TIME` : specify which of the four output columns to print
- `uuidparse -h` : display help

## See also
- uuidgen

## Links
- More information: https://manned.org/uuidparse
- tldr page: https://tldr.inbrowser.app/pages/linux/uuidparse
