# maigret

Collect OSINT information about usernames across thousands of websites. Note: The Maigret database is a local JSON file bundled with the tool.

## Usage
- `maigret username1 username2 ...` : search for accounts using one or more usernames from the Maigret database
- `maigret username --txt|--csv|--html|--xmind|--pdf|--graph|--json simple|--json ndjson` : search for accounts and generate reports in various formats
- `maigret username -p socks5://127.0.0.1:9050|http://127.0.0.1:8080` : proxy search requests through a proxy
- `maigret --web 5000` : launch Maigret Web Interface (default port `5000`)
- `maigret username --top-sites 500` : limit search to a certain number of top-ranked sites from the Maigret database
- `maigret username -a` : search all available sites in the Maigret database
- `maigret username --site twitter --site github` : search for accounts on specific sites only (useful for targeted or faster searches)

## See also
- sherlock

## Categories
network, databases, text

## Links
- More information: https://maigret.readthedocs.io/en/latest/usage-examples.html
- tldr page: https://tldr.inbrowser.app/pages/common/maigret
