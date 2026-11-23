# choco source

使用 Chocolatey 管理包的源。

## Usage
- `choco source list` : 列出当前可用的源
- `choco source add -n 名称 -s url` : 添加一个新的包源
- `choco source add -n 名称 -s url -u 用户名 -p 密码` : 添加包含凭据的新包源
- `choco source add -n 名称 -s url --cert 证书的路径` : 使用客户端证书添加新的包源
- `choco source enable -n 名称` : 启用一个包源
- `choco source disable -n 名称` : 禁用一个包源
- `choco source remove -n 名称` : 移除一个包源

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
