# nmap

Network exploration tool and security/port scanner. Some features (e.g. SYN scan) activate only when `nmap` is run with root privileges.

## Usage
- `nmap -v1|2|3 ip_or_hostname` : scan the top 1000 ports of a remote host with various verbosity levels
- `nmap -T5 -sn 192.168.0.0/24|ip_or_hostname1,ip_or_hostname2,...` : run a ping sweep over an entire subnet or individual hosts very aggressively
- `sudo nmap -A -iL path/to/file.txt` : enable OS detection, version detection, script scanning, and traceroute of hosts from a file
- `nmap -p port1,port2,... ip_or_host1,ip_or_host2,...` : scan a specific list of ports (use `-p-` for all ports from 1 to 65535)
- `nmap -sC -sV -oA top-1000-ports ip_or_host1,ip_or_host2,...` : perform service and version detection of the top 1000 ports using default NSE scripts, writing results (`-oA`) to output files
- `nmap --script "default and safe" ip_or_host1,ip_or_host2,...` : scan target(s) carefully using `default and safe` NSE scripts
- `nmap --script "http-*" ip_or_host1,ip_or_host2,... -p 80,443` : scan for web servers running on standard ports 80 and 443 using all available `http-*` NSE scripts
- `sudo nmap -T0 -D decoy_ip1,decoy_ip2,... --source-port 53 -f --data-length 16 -Pn ip_or_host` : attempt evading IDS/IPS detection by using an extremely slow scan (`-T0`), Decoy source addresses, fragmented packets, random data, and other methods

## See also
- hping3
- masscan
- naabu
- rustscan
- zmap

## Categories
network, security

## Links
- More information: https://nmap.org/book/man.html
- tldr page: https://tldr.inbrowser.app/pages/common/nmap
