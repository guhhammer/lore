# sqlmap

Detect and exploit SQL injection flaws.

## Usage
- `python sqlmap.py -u "http://www.example.com/vuln.php?id=1"` : run sqlmap against a single target URL
- `python sqlmap.py -u "http://www.example.com/vuln.php" --data="id=1"` : send data in a POST request (`--data` implies POST request)
- `python sqlmap.py -u "http://www.example.com/vuln.php" --data="query=foobar;id=1" --param-del=";"` : change the parameter delimiter (& is the default)
- `python sqlmap.py -u "http://www.example.com/vuln.php" --random-agent` : select a random `User-Agent` from `./txt/user-agents.txt` and use it
- `python sqlmap.py -u "http://www.example.com/vuln.php" --auth-type Basic --auth-cred "testuser:testpass"` : provide user credentials for HTTP protocol authentication

## Categories
network, databases, users

## Links
- More information: https://github.com/sqlmapproject/sqlmap/wiki/Usage
- tldr page: https://tldr.inbrowser.app/pages/common/sqlmap
- Wikipedia: https://en.wikipedia.org/wiki/Sqlmap
