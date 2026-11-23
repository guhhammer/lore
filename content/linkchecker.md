# linkchecker

Client to check HTML documents and websites for broken links.

## Usage
- `linkchecker https://example.com/` : find broken links on <https://example.com/>
- `linkchecker --check-extern https://example.com/` : also check URLs that point to external domains
- `linkchecker --ignore-url regex https://example.com/` : ignore URLs that match a specific `regex`
- `linkchecker --file-output csv/path/to/file https://example.com/` : output results to a CSV file

## Categories
network, files, text

## Links
- More information: https://linkchecker.github.io/linkchecker/man/linkchecker.html
- tldr page: https://tldr.inbrowser.app/pages/common/linkchecker
