# nginx

`nginx` web server.

## Usage
- `nginx` : start the server with the default configuration file
- `nginx -c configuration_file` : start the server with a custom configuration file
- `nginx -c configuration_file -p path/to/prefix` : start the server with a prefix for all relative paths in the configuration file
- `nginx -t` : test the configuration without affecting the running server
- `nginx -s reload` : reload the configuration by sending a signal with no downtime

## Categories
files

## Links
- More information: https://nginx.org/docs/switches.html
- tldr page: https://tldr.inbrowser.app/pages/common/nginx
- Wikipedia: https://en.wikipedia.org/wiki/Nginx
