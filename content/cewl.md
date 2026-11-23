# cewl

URL spidering tool for making a cracking wordlist from web content.

## Usage
- `cewl -d 2 -w path/to/wordlist.txt url` : create a wordlist file from the given URL up to 2 links depth
- `cewl --with-numbers -m 5 url` : output an alphanumeric wordlist from the given URL with words of minimum 5 characters
- `cewl --debug -e url` : output a wordlist from the given URL in debug mode including email addresses
- `cewl --auth_type basic|digest --auth_user username --auth_pass password url` : output a wordlist from the given URL using HTTP Basic or Digest authentication
- `cewl --proxy_host host --proxy_port port url` : output a wordlist from the given URL through a proxy

## Categories
network

## Links
- More information: https://digi.ninja/projects/cewl.php#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/cewl
