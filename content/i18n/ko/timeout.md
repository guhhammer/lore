# timeout

명령을 일정 시간 제한 내에서 실행.

## Usage
- `timeout 3s sleep 10` : `sleep 10`을 실행하고 3초 후 종료
- `timeout -s INT|HUP|KILL|... 5s sleep 10` : 시간 제한이 만료되면 명령에 signal을 전송 (`TERM`이 기본, 모든 신호 목록은 `kill -l`)
- `timeout -v 0.5s|1m|1h|1d|... 명령` : 시간 초과 시 전송된 신호를 `stderr`에 verbose 출력으로 표시
- `timeout -p 1s|1m|1h|1d|... 명령` : 시간 초과 여부와 관계없이 명령의 종료 상태를 유지
- `timeout -k 5m 30s 명령` : 초기 신호를 무시할 경우 특정 시간 후 강제 `KILL` 신호 전송
