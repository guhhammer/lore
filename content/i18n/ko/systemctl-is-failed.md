# systemctl is-failed

하나 이상의 systemd 유닛이 실패 상태인지 확인.

## Usage
- `systemctl is-failed` : 실패한 유닛이 있는지 확인
- `systemctl is-failed 유닛1 유닛2 ...` : 특정 유닛 또는 여러 유닛의 실패 여부 확인
- `systemctl is-failed 유닛 -q` : 출력 없이 종료 코드로만 확인
- `systemctl is-failed 유닛 --user` : 사용자 유닛의 실패 여부 확인
