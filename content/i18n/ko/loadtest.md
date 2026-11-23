# loadtest

선택한 HTTP 또는 WebSockets URL에 대해 부하 테스트를 실행.

## Usage
- `loadtest -c 10 --rps 200 https://example.com` : 동시 사용자 및 초당 요청 수를 지정하여 실행
- `loadtest --headers "accept:text/plain;text-html" https://example.com` : 사용자 지정 HTTP 헤더와 함께 실행
- `loadtest -m GET https://example.com` : 특정 HTTP 메서드를 사용하여 실행

## Links
- More information: https://github.com/alexfernandez/loadtest#usage
