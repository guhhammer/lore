# rc-update

OpenRC 서비스를 실행 수준에 추가 및 제거.

## Usage
- `rc-update show` : 모든 서비스와 추가된 실행 수준 나열
- `sudo rc-update add 서비스_이름 실행_수준` : 서비스를 실행 수준에 추가
- `sudo rc-update delete 서비스_이름 실행_수준` : 실행 수준에서 서비스 제거
- `sudo rc-update --all delete 서비스_이름` : 모든 실행 수준에서 서비스 제거

## Links
- More information: https://manned.org/rc-update
