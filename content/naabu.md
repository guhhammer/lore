# naabu

A fast port scanner written in Go with a focus on reliability and simplicity. Note: Some features are only activated when `naabu` is run with root privileges such as SYN scan.

## Usage
- `sudo naabu -host host` : run a SYN scan against default (top 100) ports of remote host
- `naabu -il` : display available network interfaces and public IP address of the local host
- `naabu -p - -host host` : scan all ports of the remote host (CONNECT scan without `sudo`)
- `naabu -tp 1000 -host host` : scan the top 1000 ports of the remote host
- `naabu -p 80,443,u:53 -host host` : scan TCP ports 80, 443, and UDP port 53 of the remote host
- `naabu -p 80,443 -cdn -host host` : show CDN type the remote host is using, if any
- `sudo naabu -v -host host -nmap-cli 'nmap -v -T5 -sC'` : run `nmap` from `naabu` for additional functionalities (`nmap` must be installed)

## See also
- hping3
- masscan
- nmap
- rustscan
- zmap

## Categories
network, users

## Links
- More information: https://docs.projectdiscovery.io/opensource/naabu/running
- tldr page: https://tldr.inbrowser.app/pages/common/naabu
