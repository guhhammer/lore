# notify-send

현재 데스크톱 환경의 알림 시스템을 사용하여 알림을 표시하는 도구.

## Usage
- `notify-send "Test" "This is a test"` : 제목 "Test"와 "This is a test"로 알림 표시
- `notify-send -i icon.png "Test" "This is a test"` : 사용자 지정 아이콘으로 알림 표시
- `notify-send -t 5000 "Test" "This is a test"` : 5초 동안 표시되는 알림
- `notify-send -u low|normal|critical "Test" "This is a test"` : 지정한 긴급도 수준으로 알림 표시 (기본값: normal)
- `notify-send "Test" -i google-chrome -a "Google Chrome"` : 앱 아이콘과 이름을 포함한 알림 표시

## Links
- More information: https://manned.org/notify-send
