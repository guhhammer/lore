# systemctl reset-failed

하나 이상 유닛의 "failed" 상태를 초기화.

## Usage
- `systemctl reset-failed` : 모든 유닛의 실패 상태 초기화
- `systemctl reset-failed 유닛` : 특정 유닛의 실패 상태 초기화
- `systemctl reset-failed 유닛_1 유닛_2 ...` : 여러 유닛의 실패 상태를 한 번에 초기화
