# oathtool

OATH one-time password tool.

## Usage
- `oathtool --totp -b "secret"` : generate TOTP token (behaves like Google Authenticator)
- `oathtool --totp -N "2004-02-29 16:21:42" -b "secret"` : generate a TOTP token for a specific time
- `oathtool --totp -b "secret" "token"` : validate a TOTP token

## Categories
users

## Links
- More information: https://www.nongnu.org/oath-toolkit/oathtool.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/oathtool
