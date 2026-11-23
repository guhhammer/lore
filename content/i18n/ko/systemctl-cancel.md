# systemctl cancel

시스템 매니저 또는 사용자 매니저에서 대기 중인 하나 이상의 작업을 취소.

## Usage
- `systemctl cancel 작업_id` : 숫자 ID로 작업 취소
- `systemctl cancel 작업_id1 작업_id2 ...` : 여러 작업을 한 번에 취소
- `systemctl cancel` : 모든 대기 중인 작업 취소
- `systemctl cancel 작업_id --user` : 사용자 서비스 매니저에서 작업 취소
