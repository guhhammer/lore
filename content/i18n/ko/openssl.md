# openssl

OpenSSL 암호화 도구 모음. `req`와 같은 일부 하위명령어는 별도의 사용법 문서를 가짐.

## Usage
- `openssl genpkey -algorithm rsa|ec -out 경로/대상/개인키.key -aes256` : 개인 키 생성 및 AES-256으로 출력 파일 암호화
- `openssl rsa -in 경로/대상/개인키.key -pubout -out 경로/대상/공개키.key` : `rsa`를 사용해 개인키(`private.key`)로부터 공개 키 생성
- `openssl req -new -x509 -key 경로/대상/개인키.key -out 경로/대상/인증서.crt -days 365` : 지정된 기간(365일) 동안 유효한 자체 서명 인증서 생성
- `openssl x509 -in 경로/대상/인증서.crt -out 경로/대상/인증서.pem|der -outform pem|der` : 인증서를 `.pem` 또는 `.der` 형식으로 변환
- `openssl x509 -in 경로/대상/인증서.crt -text -noout` : 인증서 상세 정보 확인
- `openssl req -new -key 경로/대상/개인키.key -out 경로/대상/요청.csr` : 인증서 서명 요청(CSR) 생성
- `openssl help` : 도움말 표시
- `openssl version` : 버전 표시

## Links
- More information: https://docs.openssl.org/master/man1/openssl/
