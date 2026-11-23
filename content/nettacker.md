# nettacker

Automate information gathering, vulnerability scanning, and eventually generate a report.

## Usage
- `nettacker --show-all-modules` : list all available modules
- `nettacker -m port_scan -i 192.168.0.1/24,owasp.org,scanme.org,...` : run a port scan on targets
- `nettacker -m port_scan -g 22,80,443,... -l path/to/targets.txt` : run a port scan on specific ports and targets listed in a file (newline separated)
- `nettacker --ping-before-scan -m port_scan,subdomain_scan,waf_scan,... -g 80,443 -i owasp.org` : run ping test before scan and then run multiple scan types on target

## Categories
network

## Links
- More information: https://nettacker.readthedocs.io/en/latest/Home/
- tldr page: https://tldr.inbrowser.app/pages/common/nettacker
