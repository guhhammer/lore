# nginx

`nginx` 웹 서버.

## Usage
- `nginx` : 기본 설정 파일로 서버 시작
- `nginx -c 설정_파일` : 사용자 정의 설정 파일로 서버 시작
- `nginx -c 설정_파일 -p 접두사/대상/상대/경로` : 설정 파일 내 모든 상대 경로에 접두사를 붙여 서버 시작
- `nginx -t` : 실행 중인 서버에 영향을 주지 않고 설정 테스트
- `nginx -s reload` : 서버 중단 없이 설정 다시 로드

## Links
- More information: https://nginx.org/docs/switches.html
