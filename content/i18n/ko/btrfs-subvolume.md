# btrfs subvolume

btrfs 서브볼륨과 스냅샷 관리.

## Usage
- `sudo btrfs su c 경로/대상/새로운_서브볼륨` : 새 빈 서브볼륨 생성
- `sudo btrfs su l 경로/대상/btrfs_파일시스템` : 지정된 파일 시스템의 모든 서브볼륨과 스냅샷 나열
- `sudo btrfs su d 경로/대상/서브볼륨` : 서브볼륨 삭제
- `sudo btrfs su sn -r 경로/대상/소스_서브볼륨 경로/대상/대상` : 기존 서브볼륨의 읽기 전용 스냅샷 생성
- `sudo btrfs su sn 경로/대상/소스_서브볼륨 경로/대상/대상` : 기존 서브볼륨의 읽기-쓰기 스냅샷 생성
- `sudo btrfs su sh 경로/대상/서브볼륨` : 서브볼륨에 대한 자세한 정보 표시

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html
