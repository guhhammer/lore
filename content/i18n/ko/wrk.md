# wrk

HTTP 벤치마킹 도구.

## Usage
- `wrk -t 12 -c 400 -d 30s "http://127.0.0.1:8080/index.html"` : `12`개의 스레드를 사용하고 `400`개의 HTTP 연결을 열어 `30`초 동안 벤치마크 실행
- `wrk -t 2 -c 5 -d 5s -H "Host: example.com" "http://example.com/index.html"` : 사용자 지정 헤더를 사용하여 벤치마크 실행
- `wrk -t 2 -c 5 -d 5s --timeout 2s "http://example.com/index.html"` : 요청 타임아웃을 `2`초로 설정하여 벤치마크 실행

## Links
- More information: https://github.com/wg/wrk#basic-usage
