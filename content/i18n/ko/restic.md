# restic

빠르고 안전한 백업 프로그램.

## Usage
- `restic init -r 경로/대상/저장소` : 지정된 로컬 디렉터리에 백업 저장소 초기화
- `restic -r 경로/대상/저장소 backup 경로/대상/폴더` : 디렉터리를 저장소에 백업
- `restic -r 경로/대상/저장소 snapshots` : 저장소에 현재 저장된 백업 스냅샷 표시
- `restic -r 경로/대상/저장소 restore latest|스냅샷_id --target 경로/대상/타겟` : 특정 백업 스냅샷을 대상 디렉터리에 복원
- `restic -r 경로/대상/저장소 restore 스냅샷_id --target 경로/대상/타겟 --include 경로/대상/복원` : 특정 백업의 특정 경로를 대상 디렉터리에 복원
- `restic forget --keep-last 1 --prune` : 저장소를 정리하고 각 고유 백업의 최신 스냅샷만 유지

## Links
- More information: https://restic.readthedocs.io/en/stable/manual_rest.html#usage-help
