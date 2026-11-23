# updog

A replacement for Python's SimpleHTTPServer. It allows uploading and downloading via HTTP/S, can set ad hoc SSL certificates, and use HTTP basic auth.

## Usage
- `updog` : start an HTTP server for the current directory
- `updog --directory /path/to/directory` : start an HTTP server for a specified directory
- `updog --port port` : start an HTTP server on a specified port
- `updog --password password` : start an HTTP server with a password (To log in, leave the username blank and enter the password in the password field)
- `updog --ssl` : enable transport encryption via SSL

## Categories
network, files, security

## Links
- More information: https://github.com/sc0tfree/updog
- tldr page: https://tldr.inbrowser.app/pages/common/updog
