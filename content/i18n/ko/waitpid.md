# waitpid

임의의 프로세스 종료를 대기.

## Usage
- `waitpid pid1 pid2 ...` : 지정된 PID의 모든 프로세스가 종료될 때까지 대기
- `waitpid -t n pid1 pid2 ...` : 최대 `n`초 동안 대기
- `waitpid -e pid1 pid2 ...` : 지정된 PID가 이미 종료된 경우에도 오류 발생하지 않음
- `waitpid -c n pid1 pid2 ...` : 지정된 프로세스 중 `n`개가 종료될 때까지 대기
- `waitpid -h` : 도움말 표시

## Links
- More information: https://manned.org/waitpid
