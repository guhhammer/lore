# rfkill

무선 장치를 활성화하거나 비활성화합니다.

## Usage
- `rfkill` : 장치 나열
- `rfkill -o ID,TYPE,DEVICE` : 열로 필터링
- `rfkill block bluetooth` : 유형별로 장치 차단 (예: bluetooth, wlan)
- `rfkill unblock wlan` : 유형별로 장치 차단 해제 (예: bluetooth, wlan)
- `rfkill -J` : JSON 형식으로 출력

## Links
- More information: https://manned.org/rfkill
