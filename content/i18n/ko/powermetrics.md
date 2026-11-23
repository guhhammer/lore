# powermetrics

CPU 사용량, 전력 및 인터럽트 wakeup 통계를 수집하고 표시.

## Usage
- `sudo powermetrics` : 기본 5초 샘플링 간격으로 CPU, 전력 및 wakeup 지표 표시
- `sudo powermetrics -i 2000 -n 10` : 2000 밀리초마다 지표를 샘플링하고 10개의 샘플을 수집한 후 종료
- `sudo powermetrics -o 경로/대상/출력파일.txt` : `stdout` 대신 파일에 출력을 저장
- `sudo powermetrics -r pid|wakeups|cputime|composite` : 지정한 방식으로 프로세스 목록 정렬(기본값: `composite`)
- `sudo powermetrics -f plist --show-usage-summary` : 머신이 처리하기 쉬운 property list 형식으로 출력하고 종료 시 사용량 요약 표시

## Links
- More information: https://keith.github.io/xcode-man-pages/powermetrics.1.html
