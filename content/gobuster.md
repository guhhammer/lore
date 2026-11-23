# gobuster

Brute-forces hidden paths on web servers and more.

## Usage
- `gobuster dir -u https://example.com/ -w path/to/file` : discover directories and files that match in the wordlist
- `gobuster dns -do example.com -w path/to/file` : discover subdomains
- `gobuster s3 -w path/to/file` : discover Amazon S3 buckets
- `gobuster vhost -u https://example.com/ -w path/to/file` : discover other virtual hosts on the server
- `gobuster fuzz -u https://example.com/?parameter=FUZZ -w path/to/file` : fuzz the value of a parameter
- `gobuster fuzz -u https://example.com/?FUZZ=value -w path/to/file` : fuzz the name of a parameter

## Categories
files, network

## Links
- More information: https://github.com/OJ/gobuster#modes
- tldr page: https://tldr.inbrowser.app/pages/common/gobuster
