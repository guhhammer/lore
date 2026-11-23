# amass enum

Find subdomains of a domain.

## Usage
- `amass enum -d domain_name` : find (passively) subdomains of a domain
- `amass enum -active -d domain_name -p 80,443,8080` : find subdomains of a domain and actively verify them attempting to resolve the found subdomains
- `amass enum -brute -d domain_name` : do a brute force search for subdomains
- `amass enum -o output_file -d domain_name` : save the results to a text file
- `amass enum -o output_file -dir path/to/directory -d domain_name` : save terminal output to a file and other detailed output to a directory
- `amass enum -list` : list all available data sources

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/amass-enum
