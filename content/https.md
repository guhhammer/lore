# https

This command is an alias of `http`.

## Usage
- `http https://example.com` : make a simple GET request (shows response headers and content)
- `http -p H|B|h|b|m|Hh|Hhb|... https://example.com` : print specific parts of the content (`H`: request headers, `B`: request body, `h`: response headers, `b`: response body, `m`: response metadata)
- `http GET|POST|HEAD|PUT|PATCH|DELETE|... --proxy http|https:http://localhost:8080|socks5://localhost:9050|... https://example.com` : specify the HTTP method when sending a request and use a proxy to intercept the request
- `http -F https://example.com 'User-Agent: Mozilla/5.0' 'Accept-Encoding: gzip'` : follow any `3xx` redirects and specify additional headers in a request
- `http -a username:password|token -A basic|digest|bearer GET|POST|... https://example.com/auth` : authenticate to a server using different authentication methods
- `http --offline GET|DELETE|... https://example.com` : simulate sending a request without actually sending it
- `http --session session_name|path/to/session.json -a username:password https://example.com/auth API-KEY:xxx` : use named sessions for persistent custom headers, auth credentials, and cookies
- `http -f POST https://example.com/upload cv@path/to/file` : upload a file to a form (the example below assumes that the form field is `<input type="file" name="cv" />`)

## Categories
network, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/https
