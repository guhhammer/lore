# wakeonlan

Wake-on-LAN(WOL) 기능이 활성화된 PC에 패킷 전송.

## Usage
- `wakeonlan 01:02:03:04:05:06` : MAC 주소를 지정하여 로컬 네트워크(255.255.255.255)의 모든 장치에 패킷 전송
- `wakeonlan 01:02:03:04:05:06 -i 192.168.178.2` : 특정 IP 주소를 통해 특정 장치로 패킷 전송
- `wakeonlan -n 01:02:03:04:05:06` : 명령어를 출력하지만 실행하지 않음 (드라이런)
- `wakeonlan -q 01:02:03:04:05:06` : 조용한 모드로 실행

## Links
- More information: https://manned.org/wakeonlan
