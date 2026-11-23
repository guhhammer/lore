# mkcert

로컬에서 신뢰할 수 있는 개발 인증서 생성.

## Usage
- `mkcert -install` : 시스템 신뢰 저장소에 로컬 CA 설치
- `mkcert example.org` : 주어진 도메인에 대한 인증서와 개인 키 생성
- `mkcert example.org myapp.dev 127.0.0.1` : 여러 도메인에 대한 인증서와 개인 키 생성
- `mkcert "*.example.it"` : 주어진 도메인과 하위 도메인에 대한 와일드카드 인증서와 개인 키 생성
- `mkcert -uninstall` : 로컬 CA 제거

## Links
- More information: https://manned.org/mkcert
