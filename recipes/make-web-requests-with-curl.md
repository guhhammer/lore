# make web requests with curl

Send GET and POST requests, JSON and files, see headers and status codes, go through a proxy.

## Steps
- `curl -sS https://example.com` : a page, without the progress bar, errors still shown
- `curl -I https://example.com` : only the headers
- `curl -s -o /dev/null -w '%{http_code}\n' https://example.com` : only the status code
- `curl -X POST -H 'Content-Type: application/json' -d '{"name":"x"}' https://api.example.com/items` : post JSON
- `curl -F 'file=@photo.jpg' https://example.com/upload` : upload a file as a form
- `curl -u NAME:password https://example.com` : with a user and password
- `curl -x http://proxy:8080 https://example.com` : through a proxy
- `curl -L -o file.zip https://example.com/file.zip` : download, following redirects

## Notes
- -v shows everything sent and received, to find out why a request fails.

## See also
- curl
- wget

## Categories
network development
