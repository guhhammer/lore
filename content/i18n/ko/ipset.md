# ipset

방화벽 규칙을 위한 IP 집합 생성.

## Usage
- `ipset create 집합_이름 hash:ip` : IP 주소를 포함할 빈 IP 집합 생성
- `ipset destroy 집합_이름` : 특정 IP 집합 삭제
- `ipset add 집합_이름 192.168.1.25` : 특정 집합에 IP 주소 추가
- `ipset del 집합_이름 192.168.1.25` : 집합에서 특정 IP 주소 삭제
- `ipset save 집합_이름 >경로/대상/ip_집합` : IP 집합 저장

## Links
- More information: https://manned.org/ipset
