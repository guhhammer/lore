# btrfs inspect-internal

btrfs 파일 시스템의 내부 정보를 쿼리.

## Usage
- `sudo btrfs i dump-s 경로/대상/파티션` : 슈퍼블록 정보 출력
- `sudo btrfs i dump-s -a 경로/대상/파티션` : 슈퍼블록 및 모든 복사본의 정보 출력
- `sudo btrfs i dump-t 경로/대상/파티션` : 파일 시스템 메타데이터 정보 출력
- `sudo btrfs i i n 경로/대상/btrfs_마운트` : `n`번째 inode의 파일 목록 출력
- `sudo btrfs i lo 논리_주소 경로/대상/btrfs_마운트` : 지정된 논리 주소에 있는 파일 목록 출력
- `sudo btrfs i t 경로/대상/파티션` : 루트, extent, csum 및 fs 트리의 통계 출력

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html
