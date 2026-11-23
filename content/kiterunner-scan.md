# kiterunner scan

A contextual web scanner for concurrently scanning API paths and web endpoints using kitebuilder wordlists. The `scan` subcommand targets one or multiple hosts with structured API requests.

## Usage
- `kiterunner scan https://example.com -A apiroutes-210228:5000` : scan a target with an Assetnote wordlist (e.g., first 5000 API routes)
- `kiterunner scan https://example.com -w path/to/wordlist.kite` : scan a target with a kitebuilder wordlist
- `kiterunner scan path/to/hosts.txt -w path/to/wordlist.kite` : scan multiple hosts from a file with a kitebuilder wordlist
- `kiterunner scan https://example.com -A apiroutes-210228:5000 -o json` : scan with an Assetnote wordlist and JSON output
- `kiterunner scan https://example.com -w path/to/wordlist.kite -x 5 -j 100` : scan with custom concurrency settings for performance
- `kiterunner scan https://example.com -w path/to/rafter.txt -d 0` : scan with a wordlist as a normal wordlist, disabling depth scanning
- `kiterunner scan https://example.com -w path/to/wordlist.kite -H "Authorization: Bearer token" --ignore-length 100-105` : scan with custom headers and ignore specific content length responses
- `kiterunner scan https://example.com -w path/to/wordlist.kite --kitebuilder-full-scan` : perform a full kitebuilder scan without phase scanning

## Categories
network, files

## Links
- More information: https://github.com/assetnote/kiterunner#usage
- tldr page: https://tldr.inbrowser.app/pages/common/kiterunner-scan
