# wcurl

A simple wrapper around `curl` to easily download files.

## Usage
- `wcurl https://example.com/index.html` : download the contents of a URL to a file indicated by the URL (`index.html` in this case)
- `wcurl -o path/to/file https://example.com/index.html` : download the contents of a URL to a file with a specified name
- `wcurl --curl-options "--progress-bar --http2" https://example.com/index.html` : download the contents of a URL, enabling progress bar and defaulting to HTTP/2
- `wcurl --curl-options "--clobber --continue-at -" https://example.com/index.html` : resume from an interrupted download

## See also
- wget
- curl

## Categories
network, files

## Links
- More information: https://curl.se/wcurl/manual.html
- tldr page: https://tldr.inbrowser.app/pages/common/wcurl
