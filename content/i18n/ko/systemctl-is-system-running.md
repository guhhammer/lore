# systemctl is-system-running

현재 시스템 상태를 확인.

## Usage
- `systemctl is-system-running` : 시스템이 정상적으로 동작 중인지 확인하고 현재 상태 출력
- `systemctl is-system-running -q` : 현재 상태를 조용히 체크 및 출력 (출력 없이, 종료 상태로만 출력)
- `systemctl is-system-running --wait` : 부팅이 완료될 때까지 기다린 후 상태 출력
