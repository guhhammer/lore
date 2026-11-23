# airshare

Transfer data between two machines in a local network.

## Usage
- `airshare code path/to/file_or_directory1 path/to/file_or_directory2 ...` : share files or directories
- `airshare code` : receive a file
- `airshare --upload code` : host a receiving server (use this to be able to upload files using the web interface)
- `airshare --upload code path/to/file_or_directory1 path/to/file_or_directory2 ...` : send files or directories to a receiving server
- `airshare --file-path code` : send files whose paths have been copied to the clipboard
- `airshare --clip-receive code` : receive a file and copy it to the clipboard

## Categories
files, network

## Links
- More information: https://airshare.readthedocs.io/en/latest/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/airshare
