# pidstat

CPU, 메모리, IO 등 시스템 리소스 사용량을 표시합니다.

## Usage
- `pidstat 2 10` : 2초 간격으로 10회 CPU 통계 표시
- `pidstat -r` : 페이지 폴트 및 메모리 사용량 표시
- `pidstat -d` : 프로세스 ID별 입출력 사용량 표시
- `pidstat -p pid` : 특정 PID에 대한 정보 표시
- `pidstat -C "fox|bird" -r -p ALL` : 명령 이름에 "fox" 또는 "bird"가 포함된 모든 프로세스의 메모리 통계 표시

## Links
- More information: https://manned.org/pidstat
