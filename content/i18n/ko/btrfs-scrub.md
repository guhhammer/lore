# btrfs scrub

btrfs 파일 시스템을 검사하여 데이터 무결성을 확인. 한 달에 한 번 스크럽 실행을 권장.

## Usage
- `sudo btrfs sc start 경로/대상/btrfs_마운트` : 스크럽 시작
- `sudo btrfs sc status 경로/대상/btrfs_마운트` : 진행 중이거나 마지막으로 완료된 스크럽 상태 보기
- `sudo btrfs sc c 경로/대상/btrfs_마운트` : 진행 중인 스크럽 취소
- `sudo btrfs sc r 경로/대상/btrfs_마운트` : 이전에 취소된 스크럽 재개
- `sudo btrfs sc start -B 경로/대상/btrfs_마운트` : 스크럽을 시작하고 완료될 때까지 기다린 후 종료
- `sudo btrfs sc start -q 경로/대상/btrfs_마운트` : 조용한 모드로 스크럽 시작 (오류나 통계를 출력하지 않음)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html
