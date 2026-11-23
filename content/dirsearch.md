# dirsearch

Web path scanner.

## Usage
- `dirsearch -u url --extensions-list` : scan a web server for common paths with common extensions
- `dirsearch -l path/to/url-list.txt -e php,jsp,aspx,...` : scan a list of web servers for common paths with given file extensions
- `dirsearch -u url --extensions-list -w path/to/url-paths1.txt,path/to/url-paths2.txt,...` : scan a web server for user-defined paths with common extensions
- `dirsearch -u url -e php --cookie cookie` : scan a web server using a cookie
- `dirsearch -u url -e php -m HEAD` : scan a web server using the `HEAD` HTTP method
- `dirsearch -u url -e php --json-report path/to/report.json` : scan a web server, saving the results to a `.json` file

## Categories
network, files, text

## Links
- More information: https://github.com/maurosoria/dirsearch#options
- tldr page: https://tldr.inbrowser.app/pages/common/dirsearch
