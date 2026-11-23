# vegeta

A utility and a library for HTTP load testing.

## Usage
- `echo "GET https://example.com" | vegeta attack -duration=30s` : launch an attack lasting 30 seconds
- `echo "GET https://example.com" | vegeta attack -insecure -duration=30s` : launch an attack on a server with a self-signed HTTPS certificate
- `echo "GET https://example.com" | vegeta attack -duration=30s -rate=10` : launch an attack with a rate of 10 requests per second
- `echo "GET https://example.com" | vegeta attack -duration=30s | vegeta report` : launch an attack and display a report
- `echo "GET https://example.com" | vegeta attack -duration=30s | vegeta plot >path/to/results.html` : launch an attack and plot the results on a graph (latency over time)
- `vegeta attack -duration=30s -targets=requests.txt | vegeta report` : launch an attack against multiple URLs from a file

## See also
- ab

## Categories
network, monitoring, development

## Links
- More information: https://github.com/tsenart/vegeta#usage-manual
- tldr page: https://tldr.inbrowser.app/pages/common/vegeta
