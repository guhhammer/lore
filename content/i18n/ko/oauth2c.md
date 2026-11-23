# oauth2c

OAuth 2.0 인증 서버와 상호작용하는 도구.

## Usage
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --client-secret 클라이언트_시크릿` : client credentials 방식으로 액세스 토큰 가져오기
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --response-types code` : authorization code flow 방식으로 토큰 가져오기
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --pkce` : PKCE를 사용하는 authorization code flow 방식으로 토큰 가져오기
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --username 사용자명 --password 비밀번호` : password credentials 방식으로 토큰 가져오기
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --refresh-token refresh_토큰` : 기존 access token 갱신
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --scopes 스코프1,스코프2` : 특정 scope를 지정하여 토큰 가져오기
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 --grant-type device_code` : device authorization flow 사용
- `oauth2c 발행자_주소 --client-id 클라이언트_아이디 -s --no-browser` : 브라우저를 실행하지 않는 silent 모드로 실행

## Links
- More information: https://github.com/cloudentity/oauth2c#usage
