# systemctl add-requires

하나 이상의 유닛에 대해 특정 타겟에 `Requires` 의존성을 추가.

## Usage
- `systemctl add-requires 타겟 유닛` : 타겟에서 유닛으로 `Requires` 의존성 추가
- `systemctl add-requires 타겟 유닛1 유닛2 ...` : 여러 개의 `Requires` 의존성을 한 번에 추가
- `systemctl add-requires 타겟 유닛 --user` : 사용자 레벨의 `Requires` 의존성 추가
