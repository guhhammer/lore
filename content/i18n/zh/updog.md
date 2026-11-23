# updog

Python SimpleHTTPServer 的替代方案。 它允许通过 HTTP/S 上传和下载，可以设置临时 SSL 证书，并使用 HTTP 基本认证。

## Usage
- `updog` : 为当前目录启动 HTTP 服务器
- `updog --directory /路径/到/目录` : 为指定目录启动 HTTP 服务器
- `updog --port 端口` : 在指定端口上启动 HTTP 服务器
- `updog --password 密码` : 使用密码启动 HTTP 服务器（要登录，请将用户名留空，并在密码字段中输入密码）
- `updog --ssl` : 通过 SSL 启用传输加密

## Links
- More information: https://github.com/sc0tfree/updog
