# gmssl

Crypto toolkit supporting SM1, SM2, SM3, SM4, SM9, and ZUC/ZUC256.

## Usage
- `gmssl sm3 path/to/file` : generate an SM3 hash for a file
- `gmssl sms4 -e -in path/to/file -out path/to/file.sms4` : encrypt a file using the SM4 cipher
- `gmssl sms4 -d -in path/to/file.sms4` : decrypt a file using the SM4 cipher
- `gmssl sm2 -genkey -out path/to/file.pem` : generate an SM2 private key
- `gmssl sm2 -pubout -in path/to/file.pem -out path/to/file.pem.pub` : generate an SM2 public key from an existing private key
- `gmssl zuc -e -in path/to/file -out path/to/file.zuc` : encrypt a file using the ZUC cipher
- `gmssl zuc -d -in path/to/file.zuc` : decrypt a file using the ZUC cipher
- `gmssl version` : display version

## Categories
files, security

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/gmssl
