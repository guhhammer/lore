# wpscan

WordPress vulnerability scanner.

## Usage
- `wpscan --update` : update the vulnerability database
- `wpscan --url url` : scan a WordPress website
- `wpscan --url url --stealthy` : scan a WordPress website, using random user agents and passive detection
- `wpscan --url url --enumerate vp --wp-content-dir remote/path/to/wp-content` : scan a WordPress website, checking for vulnerable plugins and specifying the path to the `wp-content` directory
- `wpscan --url url --proxy protocol://ip:port --proxy-auth username:password` : scan a WordPress website through a proxy
- `wpscan --url url --enumerate u` : perform user identifiers enumeration on a WordPress website
- `wpscan --url url --usernames username|path/to/usernames.txt --passwords path/to/passwords.txt threads 20` : execute a password guessing attack on a WordPress website
- `wpscan --url url --api-token token` : scan a WordPress website, collecting vulnerability data from the WPVulnDB (<https://wpvulndb.com/>)

## Categories
network, files, users

## Links
- More information: https://github.com/wpscanteam/wpscan
- tldr page: https://tldr.inbrowser.app/pages/common/wpscan
