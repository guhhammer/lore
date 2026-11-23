# masscan

A very fast network scanner. Works best with elevated privileges. For help with Nmap compatibility, run `masscan --nmap`.

## Usage
- `masscan ip_address|network_prefix -p 80` : scan an IP or network subnet for port 80
- `masscan 10.0.0.0/16 --top-ports 100 --rate 100000` : scan a class B subnet for the top 100 ports at 100,000 packets per second
- `masscan 10.0.0.0/16 --top-ports 100 --excludefile path/to/file` : scan a class B subnet avoiding ranges from a specific exclude file
- `masscan 10.0.0.0/16 -p 22,80 --banners --rate 100000` : scan a class B subnet with Nmap-like version detection (banner grabbing)
- `masscan 0.0.0.0/0 -p 80,443 --rate 10000000` : scan the Internet for web servers running on port 80 and 443
- `masscan 0.0.0.0/0 -p U:53 --rate 10000000` : scan the Internet for DNS servers running on UDP port 53
- `masscan 0.0.0.0/0 -p 0-65535 --output-format binary|grepable|json|list|xml --output-filename path/to/file` : scan the Internet for a specific port range and export to a file
- `masscan --readscan path/to/file` : read binary scan results from a file and output to `stdout`

## See also
- hping3
- naabu
- nmap
- rustscan
- zmap

## Categories
network, files

## Links
- More information: https://manned.org/masscan
- tldr page: https://tldr.inbrowser.app/pages/common/masscan
