# arjun

Discover HTTP parameters for web applications.

## Usage
- `arjun -u https://example.com/page.php` : scan a URL for GET parameters
- `arjun -u https://example.com/api -m POST` : scan using POST method
- `arjun -u https://example.com -o path/to/output.json` : save discovered parameters to a JSON file
- `arjun -u https://example.com -w path/to/wordlist.txt` : use a custom wordlist
- `arjun -u https://example.com -d 2` : increase request delay by specific amount of seconds to avoid rate limiting

## Categories
network, files

## Links
- More information: https://github.com/s0md3v/Arjun/wiki/Usage
- tldr page: https://tldr.inbrowser.app/pages/common/arjun
