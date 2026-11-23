# mkfs.exfat

파티션 내에 exfat 파일 시스템 생성.

## Usage
- `sudo mkfs.exfat /dev/sdb1` : 장치 b의 파티션 1 (`sdb1`) 에 exfat 파일 시스템 생성
- `sudo mkfs.exfat -L 볼륨_이름 /dev/sdXY` : 볼륨 이름을 지정하여 파일 시스템 생성
- `sudo mkfs.exfat -U 볼륨_id /dev/sdXY` : 볼륨 ID를 지정하여 파일 시스템 생성

## Links
- More information: https://manned.org/mkfs.exfat
