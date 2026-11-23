# systemctl suspend

시스템을 절전 모드로 전환.

## Usage
- `systemctl suspend` : 즉시 절전 모드로 전환
- `sleep 300 && systemctl suspend` : 5분 후 절전 모드로 전환
- `systemctl hybrid-sleep` : 절전 후 일정 시간 뒤 최대 절전으로 전환
