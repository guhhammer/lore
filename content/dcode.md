# dcode

Recursively detect and decode strings, supporting hex, decimal, binary, base64, URL, FromChar encodings, Caesar ciphers, and MD5, SHA1, and SHA2 hashes. Warning: uses 3rd-party web services for MD5, SHA1, and SHA2 hash lookups. For sensitive data, use `-s` to avoid these services.

## Usage
- `dcode "NjM3YTQyNzQ1YTQ0NGUzMg=="` : recursively detect and decode a string
- `dcode -rot 11 "spwwz hzcwo"` : rotate a string by the specified offset
- `dcode -rot all "bpgkta xh qtiitg iwpc sr"` : rotate a string by all 26 possible offsets
- `dcode -rev "hello world"` : reverse a string

## Categories
system, network, text

## Links
- More information: https://github.com/s0md3v/Decodify
- tldr page: https://tldr.inbrowser.app/pages/common/dcode
