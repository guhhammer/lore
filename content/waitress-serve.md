# waitress-serve

Pure Python WSGI HTTP Server.

## Usage
- `waitress-serve import.path:wsgi_func` : run a Python web app
- `waitress-serve --listen=localhost:8080 import.path:wsgi_func` : listen on port 8080 on localhost
- `waitress-serve --unix-socket=path/to/socket import.path:wsgi_func` : start waitress on a Unix socket
- `waitress-serve --threads=4 import.path:wsgifunc` : use 4 threads to process requests
- `waitress-serve --call import.path.wsgi_factory` : call a factory method that returns a WSGI object
- `waitress-serve --url-scheme=https import.path:wsgi_func` : use the HTTPS URL scheme

## Categories
network, files

## Links
- More information: https://docs.pylonsproject.org/projects/waitress/en/latest/runner.html
- tldr page: https://tldr.inbrowser.app/pages/common/waitress-serve
