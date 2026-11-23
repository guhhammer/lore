# pvremove

물리적 볼륨에서 LVM 레이블 제거.

## Usage
- `sudo pvremove /dev/sdXY` : 물리적 볼륨에서 LVM 레이블 제거
- `sudo pvremove -v /dev/sdXY` : 작업 중 자세한 출력 표시
- `sudo pvremove -y /dev/sdXY` : 확인을 묻지 않고 LVM 레이블 제거
- `sudo pvremove -f /dev/sdXY` : 강제로 LVM 레이블 제거
- `sudo pvremove --reportformat json /dev/sdXY` : 출력을 JSON 형식으로 표시

## Links
- More information: https://manned.org/pvremove
