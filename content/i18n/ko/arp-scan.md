# arp-scan

ARP 패킷을 호스트(IP 주소 또는 호스트 이름으로 지정)로 보내 로컬 네트워크를 검색합니다.

## Usage
- `arp-scan -l` : 현재 로컬 네트워크 검색
- `arp-scan 192.168.1.1/24` : 사용자 정의 비트마스크를 사용하여 IP 네트워크 스캔
- `arp-scan 127.0.0.0-127.0.0.31` : 사용자 정의 범위 내에서 IP 네트워크 검색
- `arp-scan 10.0.0.0:255.255.255.0` : 사용자 정의 넷 마스크로 IP 네트워크 스캔

## Links
- More information: https://github.com/royhills/arp-scan
