# biff

날짜/시간 연산, 파싱, 포맷팅을 위한 간단한 유틸리티.

## Usage
- `biff time fmt -f rfc3339 now` : 원하는 형태로 현재 시간 출력
- `biff time fmt -f '%c' now -1d 'next sat' 'last monday' '9pm last mon'` : 하나의 명령으로 여러 상대 시간 출력
- `biff time in Asia/Bangkok now | biff time round -i 15 -s minute` : 다른 타임존의 현재 시간 출력하고 15분 단위로 반올림
- `TZ='Japan' biff time in America/New_York 02:30` : 두 타임존 사이의 시간을 반환
- `biff time add -1d|1d|1w|-1m|1y|... now` : 현재를 기준으로 과거와 미래 시간을 출력
- `biff time add '1 week, 12 hours ago' now` : 여러 기간을 현재 시간에 더하기
- `biff span since 2025-01-20T12:00 -l year` : 특정 과거 시점부터 경과 시간 계산 및 단위를 반올림
- `biff tag lines /tmp/access.log | biff time in system | biff time fmt -f '%c' | head -n 3 | biff untag -s` : 로그 파일의 타임스탬프를 찾아 로컬 시간으로 변환 후 출력

## Links
- More information: https://github.com/burntsushi/biff
