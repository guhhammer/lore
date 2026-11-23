# clamscan

A virus scanner.

## Usage
- `clamscan path/to/file` : scan a file for vulnerabilities
- `clamscan -r path/to/directory` : scan all files recursively in a specific directory
- `command | clamscan -` : scan data from `stdin`
- `clamscan -d path/to/database_file_or_directory` : specify a virus database file or directory of files
- `clamscan -i` : scan the current directory and output only infected files
- `clamscan -l path/to/log_file` : print the scan report to a log file
- `clamscan --move path/to/quarantine_directory` : move infected files to a specific directory
- `clamscan --remove yes` : remove infected files

## Categories
files

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan
- tldr page: https://tldr.inbrowser.app/pages/common/clamscan
