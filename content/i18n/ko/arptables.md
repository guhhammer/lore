# arptables

`nftables` 백엔드를 사용하여 ARP 필터링 규칙을 관리하는 도구. ARP 패킷 필터링을 위한 `xtables-nft` 도구의 일부 .

## Usage
- `sudo arptables -L` : filter 테이블의 모든 ARP 규칙 목록 출력
- `sudo arptables -A INPUT -s 192.168.0.1 -j DROP` : 특정 IP 주소에서 오는 ARP 패킷을 차단하는 규칙 추가
- `sudo arptables -D INPUT 규칙_번호` : INPUT 체인에서 규칙 번호를 이용해 특정 규칙 삭제
- `sudo arptables -F` : filter 테이블의 모든 규칙 삭제
- `sudo arptables -P OUTPUT ACCEPT` : OUTPUT 체인의 기본 정책을 ACCEPT로 설정
- `sudo arptables-save >경로/대상/파일` : 현재 ARP 규칙을 파일로 저장

## Links
- More information: https://manned.org/arptables
