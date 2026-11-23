# timeshift

시스템 복원 도구.

## Usage
- `sudo timeshift --list` : 스냅샷 나열
- `sudo timeshift --check` : 새 스냅샷 생성 (예약된 경우)
- `sudo timeshift --create` : 새 스냅샷 생성 (예약되지 않은 경우에도)
- `sudo timeshift --restore` : 스냅샷 복원 (상호작용을 통해 복원할 스냅샷 선택)
- `sudo timeshift --restore --snapshot '스냅샷'` : 특정 스냅샷 복원
- `sudo timeshift --delete --snapshot '스냅샷'` : 특정 스냅샷 삭제

## Links
- More information: https://manned.org/timeshift
