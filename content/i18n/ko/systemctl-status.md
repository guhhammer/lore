# systemctl status

systemd 유닛의 상태를 출력.

## Usage
- `systemctl status 유닛.service|timer|socket|target|...` : 특정 systemd 유닛의 상태 출력
- `systemctl status --failed` : 실패한 유닛 상태 출력
- `systemctl status` : 실행 중인 모든 서비스 목록 출력
- `systemctl status -a` : 시스템의 모든 유닛 출력
- `systemctl status -t service|timer|socket|target|...` : 특정 타입의 유닛만 출력
- `systemctl status --state active|inactive|failed` : 특정 상태의 유닛만 출력
- `systemctl status 유닛 --user` : 사용자 유닛 상태 출력
