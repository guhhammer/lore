# siege

HTTP 부하 테스트 및 벤치마킹 도구.

## Usage
- `siege https://example.com` : 기본 설정으로 URL 테스트
- `siege -f 경로/대상/url_목록.txt` : URL 목록 테스트
- `siege -i -f 경로/대상/url_목록.txt` : URL 목록을 무작위 순서로 테스트 (인터넷 트래픽 시뮬레이션)
- `siege -b -f 경로/대상/url_목록.txt` : URL 목록 벤치마킹 (요청 사이에 대기하지 않음)
- `siege -c 50 -f 경로/대상/url_목록.txt` : 동시 연결 수 설정
- `siege -t 30s -f 경로/대상/url_목록.txt` : 실행 시간 설정

## Links
- More information: https://www.joedog.org/siege/manual/
