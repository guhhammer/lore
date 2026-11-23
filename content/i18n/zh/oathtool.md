# oathtool

OATH 一次性密码工具。

## Usage
- `oathtool --totp -b "密码"` : 生成 TOTP 令牌（行为类似于 Google Authenticator）
- `oathtool --totp -N "2004-02-29 16:21:42" -b "密码"` : 根据给定时间产生特定的 TOTP 令牌
- `oathtool --totp -b "密码" "令牌"` : 验证 TOTP 令牌

## Links
- More information: https://www.nongnu.org/oath-toolkit/oathtool.1.html
