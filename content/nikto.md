# nikto

Web server scanner which performs tests against web servers for multiple items.

## Usage
- `perl nikto.pl -h 192.168.0.1` : perform a basic Nikto scan against a target host
- `perl nikto.pl -h 192.168.0.1 -p 443` : specify the port number when performing a basic scan
- `perl nikto.pl -h https://192.168.0.1:443/` : scan ports and protocols with full URL syntax
- `perl nikto.pl -h 192.168.0.1 -p 80,88,443` : scan multiple ports in the same scanning session
- `perl nikto.pl -u` : update to the latest plugins and databases

## Categories
network, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/nikto
