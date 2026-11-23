# systemctl soft-reboot

커널은 유지한 채 userspace만 종료 후 재시작하는 소프트 재부팅을 수행.

## Usage
- `systemctl soft-reboot` : 즉시 소프트 재부팅 실행
- `systemctl soft-reboot -f` : 강제로 소프트 재부팅
- `systemctl soft-reboot --when "타임스탬프"` : 특정 시간에 소프트 재부팅 예약
- `systemctl soft-reboot --when cancel` : 예약된 소프트 재부팅 취소
