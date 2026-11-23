# dnstracer

Determine where a DNS gets its information from.

## Usage
- `dnstracer www.example.com` : find out where your local DNS got the information on www.example.com
- `dnstracer -s dns.example.org www.example.com` : start with a specific DNS that you already know
- `dnstracer -4 www.example.com` : only query IPv[4] servers
- `dnstracer -r 5 www.example.com` : retry each request 5 times on failure
- `dnstracer -v www.example.com` : display all steps during execution
- `dnstracer -o www.example.com` : display an overview of all received answers after execution

## Categories
network

## Links
- More information: https://manned.org/dnstracer
- tldr page: https://tldr.inbrowser.app/pages/linux/dnstracer
