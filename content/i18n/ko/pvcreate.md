# pvcreate

디스크 또는 파티션을 물리적 볼륨으로 초기화합니다.

## Usage
- `sudo pvcreate /dev/sda1` : LVM에서 사용할 수 있도록 `/dev/sda1` 볼륨 초기화
- `sudo pvcreate -f /dev/sda1` : 확인 프롬프트 없이 강제로 생성

## Links
- More information: https://manned.org/pvcreate
