# usbip

원격으로 USB 장치를 사용합니다.

## Usage
- `usbip list -l` : 모든 로컬 USB 장치와 해당 버스 ID 나열
- `systemctl start usbipd` : 서버에서 `usbip` 데몬 시작
- `sudo usbip bind -b 버스_id` : 서버에서 USB 장치를 `usbip`에 바인드
- `sudo modprobe vhci-hcd` : 클라이언트에서 `usbip`에 필요한 커널 모듈 로드
- `sudo usbip attach -r ip_주소 -b 버스_id` : 클라이언트에서 `usbip` 장치에 연결(버스 ID는 서버와 동일합니다)
- `usbip port` : 연결된 장치 나열
- `sudo usbip detach -p 포트` : 장치에서 분리
- `usbip unbind -b 버스_id` : 장치의 바인드 해제

## Links
- More information: https://manned.org/usbip
