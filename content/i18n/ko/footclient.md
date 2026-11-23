# footclient

`foot` 서버가 관리하는 새로운 터미널 창을 실행. 참고: 클라이언트를 사용하기 전에 `foot --server`를 먼저 실행해야 함.

## Usage
- `footclient` : 새로운 터미널 창을 실행하고, 창이 닫힐 때까지 대기한 후 종료 코드 반환
- `footclient -N` : 새로운 터미널 창을 실행하고 즉시 종료
- `footclient 명령어` : 새 터미널 창에서 명령어 실행
- `footclient -F` : 전체 화면 모드로 터미널 실행
- `footclient -w 너비x높이` : 지정한 창 크기(픽셀)로 터미널 실행
- `footclient -a 아이디` : 지정한 Wayland 애플리케이션 ID로 터미널 실행 (기본값: `footclient`)
- `footclient -o section.key=value` : 설정 옵션 재정의

## Links
- More information: https://manned.org/footclient
