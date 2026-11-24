# curl

curl sends a request to a URL and prints the answer: web pages, APIs,
downloads, uploads. These are the requests people make every day.

## Tasks
### Look at a page or an API
- `curl https://example.org` : print the page
- `curl -i https://example.org` : print the answer's headers too
- `curl -I https://example.org` : only the headers (a HEAD request)
- `curl -L https://example.org` : follow redirects to where the page really is
- `curl -sS https://example.org` : no progress bar, but still show errors (for scripts)
- `curl -v https://example.org` : show the whole conversation, TLS included, to see what goes wrong
### Download
- `curl -O https://example.org/file.tar.gz` : save it under its own name
- `curl -o NAME https://example.org/file` : save it as NAME
- `curl -C - -O https://example.org/big.iso` : go on with a download that stopped
- `curl -fLo NAME https://example.org/file` : fail on an HTTP error instead of saving the error page
### Send data to an API
- `curl -X POST -H 'Content-Type: application/json' -d '{"name":"x"}' https://api.example.org/items` : POST JSON
- `curl --json '{"name":"x"}' https://api.example.org/items` : the same, shorter (curl 7.82 and newer)
- `curl -d 'a=1&b=2' https://example.org/form` : POST a form
- `curl -F 'file=@photo.jpg' https://example.org/upload` : upload a file as a form
- `curl -X DELETE https://api.example.org/items/1` : another method (PUT, PATCH, DELETE)
### Log in
- `curl -u USER https://example.org` : basic auth; curl asks for the password
- `curl -H 'Authorization: Bearer TOKEN' https://api.example.org` : send a token
- `curl -c cookies.txt -b cookies.txt https://example.org` : keep cookies between requests
### Check a server
- `curl -o /dev/null -s -w '%{http_code} %{time_total}s\n' https://example.org` : only the status code and how long it took
- `curl --resolve example.org:443:10.0.0.5 https://example.org` : ask one server by its IP, keeping the name (before a DNS change)
- `curl -k https://localhost:8443` : accept a certificate that does not check out (testing only)

## Notes
- Quote URLs that hold `&` or `?`: the shell reads `&` itself.
- `-k` turns off the certificate check: never use it on the internet.
- `-d` makes the request a POST; add `-G` to send the same data in the URL of a GET.
- Never pipe a download into `sh`: read the script first, then run it.

## See also
- curl
- wget
- http

## Links
- Manual: https://curl.se/docs/manpage.html
- Everything curl: https://everything.curl.dev

## Categories
network web
