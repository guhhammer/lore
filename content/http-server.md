# http-server

Simple static HTTP server to serve static files.

## Usage
- `http-server` : start an HTTP server listening on the default port to serve the current directory
- `http-server path/to/directory -p port` : start an HTTP server on a specific port to serve a specific directory
- `http-server --username username --password password` : start an HTTP server using basic authentication
- `http-server -d false` : start an HTTP server with directory listings disabled
- `http-server -S -C path/to/cert.pem -K path/to/key.pem` : start an HTTPS server on the default port using the specified certificate
- `http-server --log-ip` : start an HTTP server and include the client's IP address in the output logging
- `http-server --cors` : start an HTTP server with CORS enabled by including the `Access-Control-Allow-Origin: *` header in all responses
- `http-server -s` : start an HTTP server with logging disabled

## Categories
network, files

## Links
- More information: https://github.com/http-party/http-server
- tldr page: https://tldr.inbrowser.app/pages/common/http-server
