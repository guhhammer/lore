# lwp-request

Simple HTTP client. Built with libwww-perl.

## Usage
- `lwp-request -m GET http://example.com/some/path` : make a simple GET request
- `lwp-request <path/to/file -m POST http://example.com/some/path` : upload a file with a POST request
- `lwp-request -H 'User-Agent: user_agent -m METHOD http://example.com/some/path` : make a request with a custom user agent
- `lwp-request -C username:password -m METHOD http://example.com/some/path` : make a request with HTTP authentication
- `lwp-request -U -m METHOD http://example.com/some/path` : make a request and print request headers
- `lwp-request -E -m METHOD http://example.com/some/path` : make a request and print response headers and status chain

## Categories
network, files, users

## Links
- More information: https://metacpan.org/pod/lwp-request
- tldr page: https://tldr.inbrowser.app/pages/common/lwp-request
