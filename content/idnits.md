# idnits

Check internet-drafts for submission nits. Looks for violations of Section 2.1 and 2.2 of the requirements listed on <https://www.ietf.org/id-info/checklist>.

## Usage
- `idnits path/to/file.txt` : check a file for nits
- `idnits --nitcount path/to/file.txt` : count nits without displaying them
- `idnits --verbose path/to/file.txt` : show extra information about offending lines
- `idnits --year 2021 path/to/file.txt` : expect the specified year in the boilerplate instead of the current year
- `idnits --doctype standard|informational|experimental|bcp|ps|ds path/to/file.txt` : assume the document is of the specified status

## Categories
files, network

## Links
- More information: https://github.com/ietf-tools/idnits
- tldr page: https://tldr.inbrowser.app/pages/common/idnits
