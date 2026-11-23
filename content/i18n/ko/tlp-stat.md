# tlp-stat

TLP 상태 보고서 생성.

## Usage
- `sudo tlp-stat` : 구성 및 모든 활성 설정으로 상태 보고서 생성
- `sudo tlp-stat --battery|disk|processor|graphics|pcie|rfkill|usb` : 다양한 장치에 대한 정보 표시
- `sudo tlp-stat -v --battery|processor|pcie|usb` : 장치에 대해 자세한 정보 표시(자세한 정보 지원 장치만)
- `sudo tlp-stat -c` : 구성 표시
- `sudo tlp-stat -P` : 전원 공급 `udev` 이벤트 모니터링
- `sudo tlp-stat --psup` : 전원 공급 진단 표시
- `sudo tlp-stat -t` : 온도 및 팬 속도 표시
- `sudo tlp-stat -s` : 일반 시스템 정보 표시

## Links
- More information: https://linrunner.de/tlp/usage/tlp-stat
