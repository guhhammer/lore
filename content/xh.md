# xh

Friendly and fast tool for sending HTTP requests. Note: `xh`, written in rust, serves as an effective drop-in replacement for `http`.

## Usage
- `xh https://postman-echo.com/get` : send a GET request (shows response headers and content)
- `xh post https://postman-echo.com/post name=john age=25` : send a POST request with a JSON body (key-value pairs are added to a top-level JSON object - e.g. `{"name": "john", "age": 25}`)
- `xh get https://postman-echo.com/response-headers foo1==bar1 foo2==bar2` : send a GET request with query parameters (e.g. <https://postman-echo.com/response-headers?foo1=bar1&foo2=bar2>)
- `xh get https://postman-echo.com header-name:header-value` : send a GET request with a custom header
- `xh -d https://example.com -o path/to/file` : make a GET request and save the response body to a file
- `xh --offline get|delete|... https://example.com` : simulate sending a request without actually sending it
- `xh --curl|curl-long --follow --verbose get https://example.com user-agent:curl` : show equivalent `curl` command (this will not send any request)

## See also
- http
- curl

## Categories
network, files

## Links
- More information: https://github.com/ducaale/xh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/xh
