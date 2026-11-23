# iptables

Linux 커널 IPv4 방화벽의 테이블, 체인 및 규칙을 구성합니다. IPv6 트래픽 규칙 설정을 위해서는 `ip6tables`를 사용하세요.

## Usage
- `sudo iptables -vnL --line-numbers` : 필터 테이블의 체인, 규칙, 패킷/바이트 카운터 및 라인 번호 보기
- `sudo iptables -P 체인 규칙` : 체인 P규칙 설정
- `sudo iptables -A 체인 -s ip_address -j 규칙` : IP에 대한 체인 정책에 규칙 A추가
- `sudo iptables -A 체인 -s ip_address -p tcp|udp|icmp|... --dport 포트 -j 규칙` : p프로토콜과 포트를 고려하여 IP에 대한 체인 정책에 규칙 A추가
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : `192.168.0.0/24` 서브넷의 모든 트래픽을 호스트의 공인 IP로 변환하는 NAT 규칙 추가
- `sudo iptables -D 체인 규칙_라인_번호` : 체인 규칙 D삭제

## Links
- More information: https://manned.org/iptables
