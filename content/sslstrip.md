# sslstrip

Perform Moxie Marlinspike's Secure Sockets Layer (SSL) stripping attacks. Perform an ARP spoofing attack in conjunction.

## Usage
- `sslstrip` : log only HTTPS POST traffic on port 10000 by default
- `sslstrip --listen=8080` : log only HTTPS POST traffic on port 8080
- `sslstrip --ssl --listen=8080` : log all SSL traffic to and from the server on port 8080
- `sslstrip --listen=8080 --all` : log all SSL and HTTP traffic to and from the server on port 8080
- `sslstrip --listen=8080 --write=path/to/file` : specify the file path to store the logs
- `sslstrip --help` : display help

## Categories
network, security, files

## Links
- More information: https://www.kali.org/tools/sslstrip/
- tldr page: https://tldr.inbrowser.app/pages/linux/sslstrip
