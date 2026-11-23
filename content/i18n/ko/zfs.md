# zfs

ZFS 파일 시스템 관리.

## Usage
- `zfs list` : 사용 가능한 모든 ZFS 파일 시스템 나열
- `zfs create 풀_이름/파일시스템_이름` : 새 ZFS 파일 시스템 생성
- `zfs destroy 풀_이름/파일시스템_이름` : ZFS 파일 시스템 삭제
- `zfs snapshot 풀_이름/파일시스템_이름@스냅샷_이름` : ZFS 파일 시스템의 스냅샷 생성
- `zfs set compression=on 풀_이름/파일시스템_이름` : 파일 시스템에 압축 활성화
- `zfs set mountpoint=/my/mount/path 풀_이름/파일시스템_이름` : 파일 시스템의 마운트 포인트 변경

## Links
- More information: https://manned.org/zfs
