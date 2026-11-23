# systemctl is-active

하나 이상의 systemd 유닛이 활성 상태인지 확인.

## Usage
- `systemctl is-active 유닛` : 유닛이 활성 상태인지 확인
- `systemctl is-active 유닛1 유닛2 ...` : 여러 유닛의 활성 상태 확인
- `systemctl is-active 유닛 -q` : 상태를 `stdout`에 출력하지 않고 확인
- `systemctl is-active 유닛 --user` : 사용자 유닛의 활성 상태 확인
