# ip neighbor

Neighbour/ARP 테이블 관리 IP 하위 명령어.

## Usage
- `ip n` : neighbour/ARP 테이블 항목 표시
- `sudo ip n f dev eth0` : `eth0` 장치에서 neighbour 테이블의 항목 제거
- `ip n g 조회_아이피 dev eth0` : neighbour 조회를 수행하고 neighbour 항목 반환
- `sudo ip n add|delete 아이피_주소 lladdr 맥_주소 dev eth0 nud reachable` : neighbour IP 주소에 대한 ARP 항목을 `eth0`에 추가하거나 삭제
- `sudo ip n change|replace 아이피_주소 lladdr 새로운_맥_주소 dev eth0` : neighbour IP 주소에 대한 ARP 항목을 `eth0`에 변경하거나 대체

## Links
- More information: https://manned.org/ip-neighbour
