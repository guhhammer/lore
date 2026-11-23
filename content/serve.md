# serve

Static file serving and directory listing.

## Usage
- `serve` : start an HTTP server listening on the default port to serve the current directory
- `serve -p port path/to/directory` : start an HTTP server on a specific port to serve a specific directory
- `serve -C` : start an HTTP server with CORS enabled by including the `Access-Control-Allow-Origin: *` header in all responses
- `serve -s` : start an HTTP server on the default port rewriting all not-found requests to the `index.html` file
- `serve --ssl-cert path/to/cert.pem --ssl-key path/to/key.pem` : start an HTTPS server on the default port using the specified certificate
- `serve -c path/to/serve.json` : start an HTTP server on the default port using a specific configuration file
- `serve --help` : display help

## Categories
files, network, security

## Links
- More information: https://github.com/vercel/serve
- tldr page: https://tldr.inbrowser.app/pages/common/serve
