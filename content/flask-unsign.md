# flask-unsign

A tool to brute-force, decode, and craft `Flask` session cookies.

## Usage
- `flask-unsign -d -c cookie` : decode a Flask session cookie
- `flask-unsign -d --server url` : decode a session cookie fetched from a URL which returns a `Set-Cookie` header
- `flask-unsign -u -c cookie` : brute-force a secret key using the default flask-unsign-wordlist (requires `flask-unsign-wordlist`)
- `flask-unsign -u -c cookie -w path/to/wordlist.txt` : brute-force a secret key with a custom wordlist (use `--no-literal-eval` for unquoted entries)
- `flask-unsign -s -c "{'logged_in': False}" -S secret` : sign a new session cookie with a secret key
- `flask-unsign -s -c "{'logged_in': False}" -S secret -l` : sign a session cookie using legacy timestamp (useful for old versions)
- `flask-unsign -u -c cookie -w path/to/wordlist.txt -t threads -nE` : brute-force a session cookie with custom threads and no literal evaluation

## Categories
network

## Links
- More information: https://github.com/Paradoxis/Flask-Unsign
- tldr page: https://tldr.inbrowser.app/pages/common/flask-unsign
