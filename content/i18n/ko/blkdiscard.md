# blkdiscard

저장 장치의 디바이스 섹터를 폐기합니다. SSD에 유용합니다.

## Usage
- `blkdiscard /dev/디바이스` : 디바이스의 모든 섹터를 폐기하여 모든 데이터 제거
- `blkdiscard -s /dev/디바이스` : 디바이스의 모든 블록을 안전하게 폐기하여 모든 데이터 제거
- `blkdiscard -l 100MB /dev/디바이스` : 디바이스의 처음 100MB를 폐기

## Links
- More information: https://manned.org/blkdiscard
