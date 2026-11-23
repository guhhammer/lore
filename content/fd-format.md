# fd-format

Format filesearch results.

## Usage
- `fd --format "{/}"` : print only the filename
- `fd --format "{//}"` : print the parent directory
- `fd --format "{.}"` : print the file path without file extension
- `fd --format "{/.}"` : print only the filename without extension
- `fd --format "\{\{ {} \}\}"` : prepend a `{` and append a `}` to the results
- `fd --format "text{}"` : prepend text to the results

## Categories
documents, files

## Links
- More information: https://manned.org/fd
- tldr page: https://tldr.inbrowser.app/pages/common/fd-format
