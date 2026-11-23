# swaynag

화면 상단 바에 메시지 표시.

## Usage
- `swaynag -m "error message"` : 오류 메시지 표시
- `swaynag -t warning -m "warning message"` : 경고 메시지 표시
- `swaynag -t custom_type -m "message"` : 설정 파일에 정의된 사용자 지정 타입으로 메시지 표시
- `swaynag -f "monospace bold 9" -m "에러 메시지"` : 지정한 글꼴을 사용하여 메시지 표시
- `TERMINAL=terminal_executable swaynag -b "버튼 텍스트" 명령어 -m "에러 메시지"` : 버튼을 생성하고 클릭 시 지정한 터미널에서 명령 실행
- `swaynag -B "버튼 텍스트" 명령어 -m "에러 메시지"` : 버튼을 생성하고 클릭 시 터미널 없이 명령 실행
- `swaynag -e bottom -m "에러 메시지"` : 화면 하단에 바 표시
- `swaynag -o DP-1 -m "에러 메시지"` : 지정한 모니터에서 `swaynag` 열기

## Links
- More information: https://github.com/swaywm/sway/blob/master/swaynag/swaynag.1.scd
