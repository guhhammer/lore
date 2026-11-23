# dirb

Scan HTTP-based webservers for directories and files.

## Usage
- `dirb https://example.org` : scan a webserver using the default wordlist
- `dirb https://example.org path/to/wordlist.txt` : scan a webserver using a custom wordlist
- `dirb https://example.org -r` : scan a webserver non-recursively
- `dirb https://example.org -a user_agent_string -c cookie_string` : scan a webserver using a specified user-agent and cookie for HTTP-requests

## Categories
network, files

## Links
- More information: https://manned.org/dirb
- tldr page: https://tldr.inbrowser.app/pages/linux/dirb
