# wget

Download files from the Web. Supports HTTP, HTTPS, and FTP.

## Usage
- `wget https://example.com/foo` : download the contents of a URL to a file (named "foo" in this case)
- `wget -O path/to/file https://example.com/foo` : download the contents of a URL to a specific file
- `wget -pkw 3 https://example.com/some_page.html` : download a single web page and all its resources with 3-second intervals between requests (scripts, stylesheets, images, etc.)
- `wget -mnp https://example.com/some_path/` : download all listed files within a directory and its sub-directories (does not download embedded page elements)
- `wget --limit-rate 300k -t 100 https://example.com/some_path/` : limit the download speed and the number of connection retries
- `wget --user username --password password https://example.com` : download a file from an HTTP server using Basic Auth (also works for FTP)
- `wget -c https://example.com` : continue an incomplete download
- `wget -P path/to/directory -i path/to/URLs.txt` : download all URLs stored in a text file to a specific directory

## See also
- wcurl
- curl

## Categories
network

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
- tldr page: https://tldr.inbrowser.app/pages/common/wget
- Wikipedia: https://en.wikipedia.org/wiki/Wget
