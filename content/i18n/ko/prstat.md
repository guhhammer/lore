# prstat

활성 프로세스 통계를 보고합니다.

## Usage
- `prstat` : 모든 프로세스 검토 및 CPU 사용량으로 정렬해 통계 보고
- `prstat -s rss` : 모든 프로세스 검토 및 메모리 사용량으로 정렬해 통계 보고
- `prstat -t` : 각 사용자에 대한 총 사용량 요약 보고
- `prstat -m` : 마이크로스테이트 프로세스 계정 정보 보고
- `prstat -c -n 5 -s cpu 1` : 매 초마다 상위 5개 CPU 사용 프로세스 목록 출력

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
