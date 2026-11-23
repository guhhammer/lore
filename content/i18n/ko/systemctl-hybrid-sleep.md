# systemctl hybrid-sleep

suspend-to-RAM과 최대 절전을 결합한 하이브리드 절전 모드로 전환.

## Usage
- `systemctl hybrid-sleep` : 즉시 하이브리드 절전 모드로 전환
- `systemctl hybrid-sleep -f` : inhibitor가 있어도 강제로 하이브리드 절전 모드로 전환
- `systemctl hybrid-sleep --no-wall` : 로그인 사용자에게 알림 없이 하이브리드 절전 모드로 전환
