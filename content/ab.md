# ab

Apache HTTP server benchmarking tool.

## Usage
- `ab -n 100 url` : execute 100 HTTP GET requests to a given URL
- `ab -n 100 -c 10 url` : execute 100 HTTP GET requests, in concurrent batches of 10, to a URL
- `ab -n 100 -T application/json -p path/to/file.json url` : execute 100 HTTP POST requests to a URL, using a JSON payload from a file
- `ab -k url` : use HTTP keep-Alive, i.e. perform multiple requests within one HTTP session
- `ab -t 60 url` : set the maximum number of seconds (timeout) to spend for benchmarking (30 by default)
- `ab -e path/to/file.csv` : write the results to a CSV file

## Categories
network, files, text

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
- tldr page: https://tldr.inbrowser.app/pages/common/ab
