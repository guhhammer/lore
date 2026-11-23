# udisksctl

`udisksd`와 상호 작용하여 스토리지 장치를 조회하고 조작.

## Usage
- `udisksctl status` : 디스크 드라이브 및 블록 장치에 대한 상위 정보 표시
- `udisksctl info -b /dev/sdX` : 장치에 대한 자세한 정보 표시
- `udisksctl info -b /dev/sdXN` : 장치 파티션에 대한 자세한 정보 표시
- `udisksctl mount -b /dev/sdXN` : 장치 파티션을 마운트하고 마운트 지점을 출력
- `udisksctl unmount -b /dev/sdXN` : 장치 파티션을 마운트 해제
- `udisksctl power-off -b /dev/sdX` : 장치를 안전하게 제거하기 위한 전원 종료
- `udisksctl monitor` : 데몬의 이벤트 모니터링

## Links
- More information: https://storaged.org/doc/udisks2-api/latest/udisksctl.1.html
