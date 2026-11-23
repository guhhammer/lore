# systemctl thaw

하나 이상의 일시 정지된 유닛을 다시 실행 상태로 복구(Thaw, resume). 유닛은 `systemctl freeze`로 정지 가능.

## Usage
- `systemctl thaw 유닛` : 특정 유닛 재개
- `systemctl thaw 유닛1 유닛2 ...` : 여러 유닛 재개
- `systemctl thaw '*'` : 현재 정지된 모든 유닛 재개
