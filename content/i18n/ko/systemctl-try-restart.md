# systemctl try-restart

현재 실행 중인 경우에만 하나 이상의 유닛을 재시작.

## Usage
- `systemctl try-restart 유닛` : 실행 중일 때만 특정 유닛 재시작
- `systemctl try-restart 유닛1 유닛2 ...` : 실행 중일 때만 여러 유닛 재시작
- `systemctl try-restart '패턴'` : 실행 중일 때만 패턴과 일치하는 모든 유닛 재시작
