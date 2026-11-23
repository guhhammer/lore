# httrack

Download websites for offline browsing.

## Usage
- `httrack https://example.com` : mirror a website
- `httrack https://example.com -O path/to/output_directory` : mirror a website to a specific directory
- `httrack https://example.com -O path/to/output_directory -i` : continue an interrupted mirror using the cache
- `httrack https://example.com -r5` : limit the recursion depth
- `httrack https://example.com -Y` : mirror all links from the first-level pages
- `httrack https://example.com "-*.zip"` : exclude ZIP files while mirroring

## Categories
network, files

## Links
- More information: https://www.httrack.com/html/fcguide.html
- tldr page: https://tldr.inbrowser.app/pages/common/httrack
