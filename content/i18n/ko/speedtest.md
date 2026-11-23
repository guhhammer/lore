# speedtest

참고: 일부 플랫폼에서는 `speedtest`를 `speedtest-cli`에 연결합니다. 이 페이지의 일부 예제가 작동하지 않는 경우, `speedtest-cli`를 참조하세요.

## Usage
- `speedtest` : 속도 테스트 실행
- `speedtest -u auto-decimal-bits|auto-decimal-bytes|auto-binary-bits|auto-binary-bytes` : 속도 테스트를 실행하고 출력 단위를 지정
- `speedtest -f human-readable|csv|tsv|json|jsonl|json-pretty` : 속도 테스트를 실행하고 출력 형식을 지정
- `speedtest -P precision` : 속도 테스트를 실행하고 소수점 자릿수를 지정 (0에서 8까지, 기본값은 2)
- `speedtest -p yes|no` : 속도 테스트를 실행하고 진행 상황을 출력 (출력 형식이 `human-readable` 및 `json`일 때만 사용 가능)
- `speedtest -L` : 거리에 따라 정렬된 모든 `speedtest.net` 서버 나열
- `speedtest -s 서버_id` : 특정 `speedtest.net` 서버로 속도 테스트 실행

## Links
- More information: https://speedtest.net
