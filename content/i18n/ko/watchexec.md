# watchexec

파일이 변경될 때 임의의 명령을 실행.

## Usage
- `watchexec ls -la` : 현재 디렉토리의 파일이 변경될 때 `ls -la` 실행
- `watchexec -e js,css,html make` : 현재 디렉토리의 JavaScript, CSS, HTML 파일이 변경될 때 `make` 실행
- `watchexec --w lib -w src make` : `lib` 또는 `src` 디렉토리의 파일이 변경될 때 `make` 실행
- `watchexec -r --stop-signal SIGKILL my_server` : 현재 디렉토리의 파일이 변경될 때 `my_server` 호출/재시작, 자식 프로세스를 중지하기 위해 `SIGKILL` 신호 전송

## Links
- More information: https://manned.org/watchexec
