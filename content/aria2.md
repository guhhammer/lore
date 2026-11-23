# aria2

This command is an alias of `aria2c`.

## Usage
- `aria2c "url"` : download a specific URI to a file
- `aria2c -o path/to/file "url"` : download a file from a URI with a specific output name
- `aria2c -Z "url1" "url2" ...` : download multiple different files in parallel
- `aria2c --checksum sha-256=hash "url1" "url2" ...` : download the same file from different mirrors and verify the checksum of the downloaded file
- `aria2c -i path/to/file -j number_of_downloads` : download the URIs listed in a file with a specific number of parallel downloads
- `aria2c -s number_of_connections "url"` : download with multiple connections
- `aria2c --ftp-user username --ftp-passwd password "url"` : download a file via FTP with a username and password
- `aria2c --max-download-limit speed "url"` : limit download speed in bytes/s

## Categories
network, files, security

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/aria2
