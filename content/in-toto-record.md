# in-toto-record

Create a signed link metadata file to provide evidence for supply chain steps.

## Usage
- `in-toto-record start -n path/to/edit_file1 path/to/edit_file2 ... --signing-key path/to/key_file -m .` : start the record (creates a preliminary link file)
- `in-toto-record stop -n path/to/edit_file1 path/to/edit_file2 ... --signing-key path/to/key_file -p .` : stop the record (expects a preliminary link file)

## Categories
files, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/in-toto-record
