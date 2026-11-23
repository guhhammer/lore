# mkfs

하드 디스크 파티션에 리눅스 파일 시스템 구축. 이 명령어는 파일 시스템이 정해진 mkfs.<type>를 위해 더 이상 사용되지 않습니다.

## Usage
- `sudo mkfs 경로/대상/파티션` : 파티션에 Linux ext2 파일 시스템 구축
- `sudo mkfs -t ext4 경로/대상/파티션` : 지정된 타입의 파일 시스템 구축
- `sudo mkfs -c -t ntfs 경로/대상/파티션` : 지정된 타입의 파일 시스템을 구축하고 불량 블록을 확인

## Links
- More information: https://manned.org/mkfs
