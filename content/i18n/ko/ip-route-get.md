# ip route get

목적지로 가는 단일 경로를 가져와 커널이 인식하는 그대로 내용을 출력합니다.

## Usage
- `ip r g 1.1.1.1` : 목적지로 가는 경로 출력
- `ip r g 목적지 from 소스` : 특정 소스 주소에서 목적지로 가는 경로 출력
- `ip r g 목적지 iif eth0` : 특정 인터페이스를 통해 도착하는 패킷의 목적지로 가는 경로 출력
- `ip r g 목적지 oif eth1` : 특정 인터페이스를 통해 강제로 출력하는 목적지로 가는 경로 출력
- `ip r g 목적지 tos 0x10` : 지정된 서비스 유형(ToS)으로 목적지로 가는 경로 출력
- `ip r g 목적지 vrf myvrf` : 특정 VRF(가상 라우팅 및 전달) 인스턴스를 사용하여 목적지로 가는 경로 출력

## Links
- More information: https://manned.org/ip-route
