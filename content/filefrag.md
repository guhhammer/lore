# filefrag

Report how badly fragmented a particular file might be.

## Usage
- `filefrag path/to/file1 path/to/file2 ...` : display a report for one or more files
- `filefrag -k path/to/file` : display a report using a 1024 byte blocksize
- `filefrag -b1024|1K|1M|1G|... path/to/file` : display a report using a certain blocksize
- `filefrag -s path/to/file1 path/to/file2 ...` : sync the file before requesting the mapping
- `filefrag -x path/to/file1 path/to/file2 ...` : display mapping of extended attributes
- `filefrag -v path/to/file1 path/to/file2 ...` : display a report with verbose information

## Categories
files, desktop

## Links
- More information: https://manned.org/filefrag
- tldr page: https://tldr.inbrowser.app/pages/linux/filefrag
