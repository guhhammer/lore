# dalfox

A powerful open-source XSS scanner focused on automation.

## Usage
- `dalfox url https://example.com` : scan a single URL for XSS vulnerabilities
- `dalfox url https://example.com -H 'X-My-Header: 123'` : scan a URL using a header for authentication
- `dalfox file path/to/file` : scan a list of URLs from a file
- `dalfox server --host 0.0.0.0 --port 8080` : start Dalfox as a REST API server

## Categories
network, files

## Links
- More information: https://dalfox.hahwul.com/reference/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/dalfox
