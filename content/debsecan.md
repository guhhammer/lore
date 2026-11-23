# debsecan

Debian Security Analyzer, a tool to list vulnerabilities on a particular Debian installation.

## Usage
- `debsecan` : list vulnerable installed packages on the current host
- `debsecan --suite release_code_name` : list vulnerable installed packages of a specific suite
- `debsecan --suite release_code_name --only-fixed` : list only fixed vulnerabilities
- `debsecan --suite sid --only-fixed --format report --mailto root --update-history` : list only fixed vulnerabilities of unstable ("sid") and mail to root
- `sudo apt upgrade $(debsecan --only-fixed --format packages)` : upgrade vulnerable installed packages

## Categories
packages, network

## Links
- More information: https://manned.org/debsecan
- tldr page: https://tldr.inbrowser.app/pages/linux/debsecan
