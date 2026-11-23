# trafilatura

A Python tool for web scraping and crawling that extracts main text, metadata, and comments from web pages. Designed for creating text corpora and extracting structured content.

## Usage
- `trafilatura -u url` : extract text from a URL
- `trafilatura -u url -o path/to/output.txt` : extract text and save to a file
- `trafilatura -u url --json` : extract text in JSON format
- `trafilatura -i path/to/url_list.txt` : extract text from multiple URLs listed in a file
- `trafilatura --sitemap url_to_sitemap.xml` : crawl a website using its sitemap
- `trafilatura -u url --formatting` : extract text while preserving HTML formatting
- `trafilatura -u url --with-comments` : extract text including comments
- `trafilatura -h` : display help

## Categories
text, network, archives

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/trafilatura
