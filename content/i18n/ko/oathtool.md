# oathtool

OATH 일회성 비밀번호 도구.

## Usage
- `oathtool --totp -b "비밀"` : TOTP 토큰 생성 (Google Authenticator처럼 동작)
- `oathtool --totp -N "2004-02-29 16:21:42" -b "비밀"` : 특정 시간에 대한 TOTP 토큰 생성
- `oathtool --totp -b "비밀" "토큰"` : TOTP 토큰 검증

## Links
- More information: https://www.nongnu.org/oath-toolkit/oathtool.1.html
