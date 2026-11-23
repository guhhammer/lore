# stdbuf

표준 스트림에 대한 버퍼링 작업을 수정하여 명령을 실행.

## Usage
- `stdbuf -i 512K 명령` : `stdin` 버퍼 크기를 512 KiB로 변경
- `stdbuf -o L 명령` : `stdout` 버퍼를 라인 버퍼로 변경
- `stdbuf -e 0 명령` : `stderr` 버퍼를 버퍼링하지 않도록 변경
