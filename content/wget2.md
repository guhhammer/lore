# wget2

An improved version of `wget` for downloading files from the web. Supports HTTP, HTTPS, and HTTP/2 protocols with enhanced performance. By default, `wget2` uses multiple threads for faster downloads.

## Usage
- `wget2 https://example.com/resource` : download the contents of a URL to a file using multiple threads (default behavior differs from `wget`)
- `wget2 --max-threads 10 https://example.com/resource` : limit the number of threads used for downloading (default is 5 threads)
- `wget2 -p -k https://example.com/somepage.html` : download a single web page and all its resources (scripts, stylesheets, images, etc.)
- `wget2 -m -np https://example.com/somepath/` : mirror a website, but do not ascend to the parent directory (does not download embedded page elements)
- `wget2 --limit-rate 300k -t 100 https://example.com/somepath/` : limit the download speed and the number of connection retries
- `wget2 -c https://example.com` : continue an incomplete download (behavior is consistent with `wget`)
- `wget2 -P path/to/directory -i URLs.txt` : download all URLs stored in a text file to a specific directory
- `wget2 --user username --password password https://example.com` : download a file from an HTTP server using Basic Auth (also works for HTTPS)

## Categories
network, files, monitoring

## Links
- More information: https://manned.org/wget2
- tldr page: https://tldr.inbrowser.app/pages/common/wget2
