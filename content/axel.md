# axel

Download accelerator. Supports HTTP, HTTPS, FTP, and FTPs.

## Usage
- `axel url` : download a URL to a file
- `axel url -o path/to/file` : download and specify an output file
- `axel -n number url` : download with a specific number connections
- `axel -Snumber url` : use a specific number of mirrors for searching and downloading
- `axel -s speed url` : limit download speed (bytes per second)
- `axel -4 url` : use the IPv4 protocol only when connecting to the host
- `axel -q -U "Mozilla/5.0" url` : limit output to `stdout` and use a custom user-agent when downloading

## See also
- aria2c

## Categories
network, files

## Links
- More information: https://manned.org/axel
- tldr page: https://tldr.inbrowser.app/pages/common/axel
