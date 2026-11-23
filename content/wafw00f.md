# wafw00f

Identify and fingerprint Web Application Firewall (WAF) products protecting a website.

## Usage
- `wafw00f https://www.example.com` : check if a website is using any WAF
- `wafw00f -a https://www.example.com` : test for all detectable WAFs without stopping at the first match
- `wafw00f -p http://localhost:8080 https://www.example.com` : pass requests through a proxy (such as BurpSuite)
- `wafw00f -t Cloudflare|Cloudfront|Fastly|ZScaler|... https://www.example.com` : test for a specific WAF product (run `wafw00f --list` to get list of all supported WAFs)
- `wafw00f -H path/to/headers.txt https://www.example.com` : pass custom headers from a file
- `wafw00f -i path/to/urls.txt -vv` : read target inputs from a file and show verbose output (multiple `v` for more verbosity)
- `wafw00f -l` : list all WAFs that can be detected

## Categories
network, files, security

## Links
- More information: https://github.com/EnableSecurity/wafw00f/wiki/Usage#arguments-list
- tldr page: https://tldr.inbrowser.app/pages/common/wafw00f
