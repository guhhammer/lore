# cupstestppd

Test conformance of PPD files to the version 4.3 of the specification. Error codes (1, 2, 3, and 4, respectively): bad CLI arguments, unable to open file, unskippable format errors, and non-conformance with PPD specification. Note: This command is deprecated.

## Usage
- `cupstestppd -q path/to/file1.ppd path/to/file2.ppd ...` : test the conformance of one or more files in quiet mode
- `cupstestppd <path/to/file.ppd -v -` : get the PPD file from `stdin`, showing detailed conformance testing results
- `find . -name \*.ppd \! -execdir cupstestppd -q '{}' \; -print` : test all PPD files under the current directory, printing the names of each file that does not conform

## See also
- lpadmin

## Categories
files, development

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupstestppd.html
- tldr page: https://tldr.inbrowser.app/pages/common/cupstestppd
