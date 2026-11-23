# trap

이벤트 발생 시 명령을 실행.

## Usage
- `trap` : 예상 이벤트의 이름과 명령 나열
- `trap 'echo "신호 SIGHUP 수신"' HUP` : 신호를 받았을 때 명령 실행
- `trap - HUP INT` : 명령 제거

## Links
- More information: https://manned.org/trap.1posix
