# shutdown

시스템 종료 및 재부팅.

## Usage
- `shutdown -h now` : 즉시 전원 끄기 (halt)
- `shutdown -r now` : 즉시 r재부팅
- `shutdown -r +5 &` : 5분 후 r재부팅
- `shutdown -h 13:00` : 오후 1시에 종료하기 (24시간 h 형식 사용)
- `shutdown -c` : 보류 중인 종료/재부팅 작업 c취소

## Links
- More information: https://manned.org/shutdown.8
