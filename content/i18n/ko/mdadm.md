# mdadm

RAID 관리 도구.

## Usage
- `sudo mdadm --create /dev/md/MyRAID --level raid_레벨 --raid-devices 디스크_개수 /dev/sdXN` : 배열 생성
- `sudo mdadm --stop /dev/md0` : 배열 중지
- `sudo mdadm --fail /dev/md0 /dev/sdXN` : 디스크를 실패로 표시
- `sudo mdadm --remove /dev/md0 /dev/sdXN` : 디스크 제거
- `sudo mdadm --assemble /dev/md0 /dev/sdXN` : 배열에 디스크 추가
- `sudo mdadm --detail /dev/md0` : RAID 정보 표시
- `sudo mdadm --zero-superblock /dev/sdXN` : 디스크의 RAID 메타데이터 삭제하여 초기화

## Links
- More information: https://manned.org/mdadm
