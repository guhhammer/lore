# ip link

네트워크 인터페이스 관리.

## Usage
- `ip l` : 모든 네트워크 인터페이스 정보 표시
- `ip l sh ethN` : 특정 네트워크 인터페이스 정보 표시
- `sudo ip l s ethN up|down` : 네트워크 인터페이스 활성화 또는 비활성화
- `sudo ip l s ethN al "LAN 인터페이스"` : 네트워크 인터페이스에 의미 있는 이름 부여
- `sudo ip l s ethN a ff:ff:ff:ff:ff:ff` : 네트워크 인터페이스의 MAC 주소 변경
- `sudo ip l s ethN mtu 9000` : 네트워크 인터페이스의 MTU 크기를 변경하여 점보 프레임 사용

## Links
- More information: https://manned.org/ip-link
