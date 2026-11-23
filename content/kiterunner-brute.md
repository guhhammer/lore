# kiterunner brute

A contextual web scanner for bruteforcing API paths and web endpoints using wordlists. The `brute` subcommand targets one or multiple hosts.

## Usage
- `kiterunner brute https://example.com -A apiroutes-210328:20000` : bruteforce a target with an Assetnote wordlist (e.g., first 20,000 API routes)
- `kiterunner brute https://example.com -w path/to/wordlist.txt` : bruteforce a target with a custom wordlist
- `kiterunner brute https://example.com -w path/to/dirsearch.txt -D -e json,txt` : bruteforce using a dirsearch-style wordlist with extension substitution
- `kiterunner brute https://example.com -w path/to/wordlist.txt -e aspx,ashx -o json` : bruteforce with specific file extensions appended and output in JSON format
- `kiterunner brute path/to/targets.txt -w path/to/wordlist.txt -x 5 -j 100` : bruteforce a list of targets from a file with custom concurrency settings for performance
- `kiterunner brute https://example.com -w path/to/wordlist.txt --ignore-length 100-105` : bruteforce and ignore specific content length responses
- `kiterunner brute https://example.com -w path/to/wordlist.txt -H "Authorization: Bearer token"` : bruteforce with custom HTTP headers
- `kiterunner brute path/to/targets.txt -w path/to/wordlist.txt --fail-status-codes 400,401,404` : bruteforce a list of targets from a file with fail status code filtering

## Categories
files, network, text

## Links
- More information: https://github.com/assetnote/kiterunner#usage
- tldr page: https://tldr.inbrowser.app/pages/common/kiterunner-brute
