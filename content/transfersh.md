# transfersh

An unofficial client for transfer.sh.

## Usage
- `transfersh path/to/file` : upload a file to transfer.sh
- `transfersh -p path/to/file` : upload a file showing a progress bar (requires Python package `requests_toolbelt`)
- `transfersh -n filename path/to/file` : upload a file using a different file name
- `transfersh -sn upload.server.name path/to/file` : upload a file to a custom transfer.sh server
- `transfersh -r path/to/directory/` : upload all files from a directory recursively
- `transfersh -rt path/to/directory` : upload a specific directory as an uncompressed tar

## Categories
files, network

## Links
- More information: https://github.com/AlpixTM/transfersh
- tldr page: https://tldr.inbrowser.app/pages/common/transfersh
