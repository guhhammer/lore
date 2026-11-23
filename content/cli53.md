# cli53

Tool for Amazon Route 53.

## Usage
- `cli53 list` : list domains
- `cli53 create example.com --comment "comment"` : create a domain
- `cli53 export example.com` : export a bind zone file to `stdout`
- `cli53 rc example.com 'www 300 CNAME lb'` : create a `www` subdomain pointing to a relative record in the same zone
- `cli53 rc example.com 'www 300 CNAME lb.example.com.'` : create a `www` subdomain pointing to an external address (must end with a dot)
- `cli53 rc example.com 'www 300 A 150.130.110.1'` : create a `www` subdomain pointing to an IP address
- `cli53 rc --replace 'www 300 A 150.130.110.2'` : replace a `www` subdomain pointing to a different IP
- `cli53 rd example.com www A` : delete a record A

## Categories
network

## Links
- More information: https://github.com/barnybug/cli53
- tldr page: https://tldr.inbrowser.app/pages/common/cli53
