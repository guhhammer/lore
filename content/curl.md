# curl

Transfer data from or to a server. Supports most protocols, including HTTP, HTTPS, FTP, SCP, etc.

## Usage
- `curl https://example.com` : make an HTTP GET request and dump the contents in `stdout`
- `curl -L -D - https://example.com` : make an HTTP GET request, follow any `3xx` redirects, and dump the reply headers and contents to `stdout`
- `curl -O https://example.com/filename.zip` : download a file, saving the output under the filename indicated by the URL
- `curl -X POST -d 'name=bob' http://example.com/form` : send form-encoded data (POST request of type `application/x-www-form-urlencoded`). Use `--data @file_name` or `--data @'-'` to read from `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : send a request with an extra header, using a custom HTTP method and over a proxy (such as BurpSuite), ignoring insecure self-signed certificates
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : send data in JSON format, specifying the appropriate Content-Type header
- `curl -E client.pem --key key.pem -k https://example.com` : pass client certificate and private key for the request, skipping certificate validation
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : resolve a hostname to a custom IP address, with verbose output (similar to editing the `/etc/hosts` file for custom DNS resolution)

## See also
- wcurl
- wget

## Categories
network

## Links
- More information: https://curl.se/docs/manpage.html
- tldr page: https://tldr.inbrowser.app/pages/common/curl
- Wikipedia: https://en.wikipedia.org/wiki/CURL
