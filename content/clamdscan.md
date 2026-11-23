# clamdscan

Scan for viruses using the ClamAV Daemon.

## Usage
- `clamdscan path/to/file_or_directory` : scan a file or directory for vulnerabilities
- `command | clamdscan -` : scan data from `stdin`
- `clamdscan --infected` : scan the current directory and output only infected files
- `clamdscan --log path/to/log_file` : print the scan report to a log file
- `clamdscan --move path/to/quarantine_directory` : move infected files to a specific directory
- `clamdscan --remove` : remove infected files
- `clamdscan --multiscan` : use multiple threads to scan a directory
- `clamdscan --fdpass` : pass the file descriptor instead of streaming the file to the daemon

## Categories
files, system, monitoring

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan
- tldr page: https://tldr.inbrowser.app/pages/common/clamdscan
