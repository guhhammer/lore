# iotop

프로세스 또는 스레드별 현재 I/O 사용량을 표 형태로 표시하는 도구.

## Usage
- `sudo iotop` : top과 비슷한 I/O 모니터 실행
- `sudo iotop -o` : 실제로 I/O를 수행 중인 프로세스/스레드만 표시
- `sudo iotop -b` : 비대화식 모드로 I/O 사용량 출력
- `sudo iotop -P` : 프로세스의 I/O 사용량만 표시 (기본값은 모든 스레드 표시)
- `sudo iotop -p pid` : 특정 PID의 I/O 사용량 표시
- `sudo iotop -u user` : 특정 사용자 기준으로 I/O 사용량 표시
- `sudo iotop -a` : 대역폭 대신 누적 I/O 사용량 표시

## Links
- More information: https://manned.org/iotop
