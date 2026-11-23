# openssl req

PKCS#10 인증서 서명 요청을 관리하는 OpenSSL 명령어.

## Usage
- `openssl req -new -sha256 -key 파일이름.key -out 파일이름.csr` : 인증 기관에 보낼 인증서 서명 요청 생성
- `openssl req -new -x509 -newkey rsa:4096 -keyout 파일이름.key -out 파일이름.cert -subj "/C=XX/CN=foobar" -days 365` : 자체 서명된 인증서와 해당 키 쌍을 생성하여 파일에 저장

## Links
- More information: https://docs.openssl.org/master/man1/openssl-req/
