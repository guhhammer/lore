# sipcalc

IPv4 및 IPv6 주소의 IP 서브넷 정보를 계산.

## Usage
- `sipcalc 192.168.1.0/24` : IPv4 CIDR 주소의 서브넷 정보를 표시
- `sipcalc -a 192.168.1.0/24` : 주소에 대한 모든 정보 표시
- `sipcalc -s 255.255.255.128 192.168.1.0/24` : 지정한 서브넷 마스크로 IPv4 네트워크를 분할
- `sipcalc -r 2001:db8::/32` : IPv6 역방향 DNS 정보를 표시
- `sipcalc -d 192.168.1.0/24` : DNS 이름 확인을 활성화
- `sipcalc -h` : 도움말 표시

## Links
- More information: https://manned.org/sipcalc
