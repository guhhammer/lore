# journalctl

systemd 저널을 조회합니다.

## Usage
- `journalctl -b -p 3` : 현재 부트에서 우선순위 레벨 3(오류)로 모든 메시지 표시
- `journalctl --vacuum-time 2d` : 2일 이상된 저널 로그 삭제
- `journalctl -n N -f` : 마지막 N줄만 표시하고 새 메시지를 팔로우(전통적인 syslog의 `tail -f`처럼) 표시
- `journalctl -u 유닛` : 특정 유닛의 모든 메시지 표시
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID 유닛)` : 마지막으로 시작된 이후 유닛의 로그 표시
- `journalctl -S now|today|yesterday|tomorrow -U "YYYY-MM-DD HH:MM:SS"` : 시간 범위 내의 메시지 필터링(타임스탬프 또는 "yesterday" 같은 플레이스홀더 사용 가능)
- `journalctl _PID=pid` : 특정 프로세스의 모든 메시지 표시
- `journalctl 경로/대상/실행_파일` : 특정 실행 파일의 모든 메시지 표시

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
