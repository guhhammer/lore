# tput

터미널 설정 및 기능을 조회하고 수정.

## Usage
- `tput cup 행 열` : 커서를 특정 화면 위치로 이동
- `tput setaf|setab ansi_색상_코드` : 전경색(af) 또는 배경색(ab) 설정
- `tput rev` : 텍스트와 배경 색상을 반전
- `tput sgr0` : 모든 터미널 속성 초기화
- `tput cols|lines|colors` : 열 수, 줄 수 또는 색상 수를 표시
- `tput smam|rmam` : 자동 줄바꿈 활성화 또는 비활성화
- `tput civis|cnorm` : 터미널 커서를 숨기거나 표시
- `tput smcup|rmcup` : 터미널 텍스트 상태를 저장하거나 복원 (smcup은 스크롤 휠 이벤트도 캡처)

## Links
- More information: https://manned.org/tput
