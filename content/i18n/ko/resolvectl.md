# resolvectl

도메인 이름, IPv4 및 IPv6 주소, DNS 리소스 레코드 및 서비스를 해석. DNS 해석기를 검사하고 재구성.

## Usage
- `resolvectl` : DNS 설정 표시
- `resolvectl query 도메인1 도메인2 ...` : 하나 이상의 도메인에 대한 IPv4 및 IPv6 주소 해석
- `resolvectl query ip_주소` : 지정한 IP 주소의 도메인 검색
- `resolvectl flush-caches` : 모든 로컬 DNS 캐시 플러시
- `resolvectl statistics` : DNS 통계(트랜잭션, 캐시 및 DNSSEC 판결) 표시
- `resolvectl --legend no -t MX query 도메인` : 도메인의 MX 레코드 검색
- `resolvectl service _서비스._프로토콜 이름` : 예를 들어 _xmpp-server._tcp gmail.com와 같은 SRV 레코드 해석
- `resolvectl tlsa tcp 도메인:443` : TLS 키 검색

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html
