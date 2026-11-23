# systemctl hibernate

현재 상태를 디스크에 저장한 뒤, 시스템 전원을 끄는 최대 절전 모드로 전환.

## Usage
- `systemctl hibernate` : 즉시 최대 절전 모드로 전환
- `systemctl hibernate -f` : inhibitors가 있어도 강제로 최대 절전 모드로 전환
- `systemctl hibernate --no-wall` : 로그인 사용자에게 알림 없이 최대 절전 모드로 전환
