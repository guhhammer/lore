# rustscan

Modern Port Scanner written in Rust. Note: `nmap` must be installed for some of the examples below to work.

## Usage
- `rustscan -a ip_or_hostname1,ip_or_hostname2,...` : scan all ports of one or more comma-delimited addresses using the default values
- `rustscan --top -a address` : scan the top 1000 ports with service and version detection
- `rustscan -p port1,port2,... -a address` : scan a specific list of ports
- `rustscan -r start-end -a address` : scan a specific range of ports
- `rustscan -a address -- -O -sC` : invoke `nmap` functionalities (Nmap's OS detection and default scripts)
- `rustscan -b batch_size -t timeout -a address` : scan with custom batch size (default: 4500) and timeout (default: 1500ms)
- `rustscan --scan-order serial|random -a address` : scan with specific port order
- `rustscan -g -a address` : scan in greppable mode (only output of the ports, no `nmap`)

## See also
- hping3
- masscan
- naabu
- nmap
- zmap

## Categories
network

## Links
- More information: https://github.com/bee-san/RustScan/wiki
- tldr page: https://tldr.inbrowser.app/pages/common/rustscan
