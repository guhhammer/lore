# traceroute

네트워크 호스트로 패킷 경로를 추적하여 출력.

## Usage
- `traceroute example.com` : 호스트로 traceroute 실행
- `traceroute -n example.com` : IP 주소 및 호스트 이름 매핑 비활성화
- `traceroute -w 0.5 example.com` : 응답 대기 시간을 초 단위로 지정
- `traceroute -q 5 example.com` : 홉당 쿼리 수 지정
- `traceroute example.com 42` : 프로빙 패킷의 크기를 바이트 단위로 지정
- `traceroute --mtu example.com` : 목적지까지의 MTU 결정
- `traceroute -I example.com` : UDP 대신 ICMP를 사용하여 traceroute 실행

## Links
- More information: https://manned.org/traceroute
