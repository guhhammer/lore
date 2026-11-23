# mkfs.bcachefs

파티션 내에 `bcachefs` 파일 시스템 생성.

## Usage
- `sudo mkfs.bcachefs /dev/sdX1` : 장치 (`X`) 의 파티션 1에 `bcachefs` 파일 시스템 생성
- `sudo mkfs.bcachefs -L 볼륨_레이블 /dev/sdX1` : 볼륨 레이블을 사용하여 `bcachefs` 파일 시스템 생성

## Links
- More information: https://bcachefs-docs.readthedocs.io/en/latest/mgmt-formatting.html
