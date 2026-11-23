# mkfs.btrfs

BTRFS 파일 시스템 생성. 기본값은 `raid1`로, 데이터 블록의 두 복사본이 두 개의 다른 장치에 분산됩니다.

## Usage
- `sudo mkfs.btrfs --metadata single --data single /dev/sda` : 단일 장치에 btrfs 파일 시스템 생성
- `sudo mkfs.btrfs --metadata raid1 --data raid1 /dev/sda /dev/sdb /dev/sdN` : 여러 장치에 raid1으로 btrfs 파일 시스템 생성
- `sudo mkfs.btrfs --label "레이블" /dev/sda [/dev/sdN]` : 파일 시스템에 레이블 설정

## Links
- More information: https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html
