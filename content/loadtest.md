# loadtest

Run a load test on the selected HTTP or WebSockets URL.

## Usage
- `loadtest -c 10 --rps 200 https://example.com` : run with concurrent users and a specified amount of requests per second
- `loadtest --headers "accept:text/plain;text-html" https://example.com` : run with a custom HTTP header
- `loadtest -m GET https://example.com` : run with a specific HTTP method

## Categories
network, monitoring, development

## Links
- More information: https://github.com/alexfernandez/loadtest#usage
- tldr page: https://tldr.inbrowser.app/pages/common/loadtest
