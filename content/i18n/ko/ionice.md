# ionice

프로그램의 I/O 스케줄링 클래스 및 우선순위를 가져오거나 설정합니다. 스케줄링 클래스: 1 (실시간), 2 (최선 노력), 3 (유휴). 우선순위 수준: 0 (가장 높음) - 7 (가장 낮음).

## Usage
- `ionice -c 1..3 -n 0..7 명령` : 주어진 스케줄링 클래스 및 우선순위로 명령 실행
- `ionice -c 1..3 -p|P|u id` : 실행 중인 프로세스의 I/O 스케줄링 c클래스를 특정 ppid, Pgid 또는 uuid로 설정
- `ionice -t -n 0..7 -p pid` : 원하는 우선순위를 설정할 수 없는 경우에도 명령 실행 (이 경우는 권한 부족 또는 오래된 커널 버전으로 인해 발생할 수 있음)
- `ionice -p pid` : 실행 중인 프로세스의 I/O 스케줄링 클래스 및 우선순위 출력

## Links
- More information: https://manned.org/ionice
