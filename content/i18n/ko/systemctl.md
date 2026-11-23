# systemctl

systemd 시스템 및 서비스 관리자를 제어합니다.

## Usage
- `systemctl status` : 실행 중인 서비스 모두 표시
- `systemctl --failed` : 실패한 유닛 나열
- `systemctl start|stop|restart|reload|status 유닛` : 서비스 시작/중지/재시작/재로드/상태 표시
- `systemctl enable|disable 유닛` : 부팅 시 시작할 유닛 활성화/비활성화
- `systemctl daemon-reload` : systemd를 재로드하고 새 유닛 또는 변경된 유닛 검색
- `systemctl is-active|is-enabled|is-failed 유닛` : 유닛이 활성/활성화됨/실패했는지 확인
- `systemctl list-units -t service|socket|automount --state failed|running` : 실행 중이거나 실패한 상태로 필터링하여 모든 서비스/소켓/자동 마운트 유닛 나열
- `systemctl cat 유닛` : 유닛 파일의 내용 및 절대 경로 표시

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
