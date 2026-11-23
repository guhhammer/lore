# wtype

Wayland에서 키보드 입력을 시뮬레이트합니다. X11의 `xdotool type`과 유사합니다.

## Usage
- `wtype "Hello World"` : 텍스트 입력을 시뮬레이트
- `wtype -k Left` : 특정 키 입력
- `wtype -M shift|ctrl|...` : 수정 키 누르기
- `wtype -m ctrl` : 수정 키 놓기
- `wtype -d 500 -- "텍스트"` : 키 입력 간 대기 시간 설정 (밀리초)
- `echo "텍스트" | wtype -` : `stdin`에서 텍스트 읽기

## Links
- More information: https://manned.org/wtype
