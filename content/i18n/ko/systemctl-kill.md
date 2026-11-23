# systemctl kill

유닛에 속한 하나 이상의 프로세스에 시그널을 전송.

## Usage
- `systemctl kill 유닛` : 유닛을 종료하기 위해 `SIGTERM` 시그널 전송
- `systemctl kill -s 시그널_번호|시그널_이름 유닛` : 특정 시그널을 유닛에 전송
- `systemctl kill -s 1 --kill-whom main 유닛` : 유닛 메인 프로세스에만 `SIGHUP` 시그널 전송
- `systemctl kill -s help` : 사용 가능한 모든 시그널 목록 출력
