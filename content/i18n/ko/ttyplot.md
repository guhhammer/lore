# ttyplot

실시간 커맨드라인 플로팅 유틸리티로, `stdin`으로 데이터 입력을 받습니다.

## Usage
- `{ echo 1 2 3; cat; } | ttyplot` : 값 `1`, `2`, `3`을 플로팅 (`cat`은 ttyplot의 종료를 방지)
- `{ echo 1 2 3; cat; } | ttyplot -t 제목 -u 단위` : 특정 제목과 단위를 설정
- `{ while true; do echo $RANDOM; sleep 1; done } | ttyplot` : while 루프를 사용하여 랜덤 값을 지속적으로 플로팅
- `ping 8.8.8.8 | sed -u 's/^.*time=//g; s/ ms//g' | ttyplot -t "8.8.8.8로의 핑" -u ms` : `ping`의 출력을 파싱하여 시각화

## Links
- More information: https://github.com/tenox7/ttyplot
