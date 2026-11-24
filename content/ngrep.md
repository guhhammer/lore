# ngrep

Filter network traffic packets using `regex`.

## Usage
- `ngrep -d any` : capture traffic of all interfaces
- `ngrep -d eth0` : capture traffic of a specific interface
- `ngrep -d eth0 port 22` : capture traffic crossing port 22 of interface eth0
- `ngrep host www.example.com` : capture traffic from or to a host
- `ngrep -d eth0 'User-Agent:'` : filter keyword 'User-Agent:' of interface eth0

## Categories
network, text

## Links
- More information: https://github.com/jpr5/ngrep/blob/master/EXAMPLES.md
- tldr page: https://tldr.inbrowser.app/pages/common/ngrep
- Wikipedia: https://en.wikipedia.org/wiki/Ngrep
