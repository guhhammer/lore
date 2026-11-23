# transmission-daemon

`transmission-remote` 또는 웹 인터페이스로 제어되는 데몬.

## Usage
- `transmission-daemon` : 헤드리스 `transmission` 세션 시작
- `transmission-daemon -c 경로/대상/폴더` : 특정 디렉터리를 감시하여 새로운 토렌트를 시작
- `transmission-daemon -d >경로/대상/파일.json` : JSON 형식으로 데몬 설정 덤프
- `transmission-daemon -t -u 사용자명 -v 비밀번호 -p 9091 -a 127.0.0.1` : 웹 인터페이스에 대한 특정 설정으로 시작

## Links
- More information: https://manned.org/transmission-daemon
