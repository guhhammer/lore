# xdotool

X11을 위한 명령줄 자동화 도구.

## Usage
- `xdotool search --onlyvisible --name firefox` : 실행 중인 Firefox 창의 X-Windows 창 ID 검색
- `xdotool click 3` : 오`<RightClick>`
- `xdotool getactivewindow` : 현재 활성 창의 ID 가져오기
- `xdotool windowfocus --sync 12345` : ID가 12345인 창에 포커스 맞추기
- `xdotool type --delay 500 "Hello world"` : 각 글자마다 500ms 지연을 두고 메시지 입력
- `xdotool key KP_Enter` : `<Enter>` 키 누르기

## Links
- More information: https://manned.org/xdotool
