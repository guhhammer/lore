# systemctl-halt

시스템을 종료하고 정지 상태로 반환 (OS 커널은 중지되지만 하드웨어 전원은 유지됨).

## Usage
- `systemctl halt` : 시스템 정지
- `systemctl halt -f` : 서비스 종료 절차 없이 즉시 정지
- `systemctl halt -f --no-wall` : 로그인 사용자에게 알림 없이 즉시 정지
- `systemctl halt -ff` : 프로세스 종료나 파일시스템 마운트 해제 없이 즉시 정지 (위험 및 데이터 손실 가능성 있음)
- `systemctl halt --when 23:00` : 특정 시간에 맞춰 정지 예약 (예: 23:00)
- `systemctl halt --when +2h` : 일정 시간 후 정지 예약 (예: 2시간 후)
- `systemctl halt --when cancel` : 예약된 정지 취소

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html#halt
