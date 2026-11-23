# lychee

Find broken URLs.

## Usage
- `lychee https://example.com` : scan a website for broken links
- `lychee -f compact|detailed|json|junit|markdown https://example.com` : display a breakdown of error types
- `lychee --max-concurrency 5 links.txt` : limit the amount of connections to prevent DDOS protection
- `grep -r "pattern" | lychee -` : check files in a directory structure for any broken URLs
- `lychee -h` : display help

## Categories
network

## Links
- More information: https://lychee.cli.rs/guides/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/lychee
