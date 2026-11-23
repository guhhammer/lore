# wol

Wake-on-LAN 매직 패킷을 보내는 클라이언트.

## Usage
- `wol mac_주소` : 장치에 WoL 패킷 전송
- `wol -i ip_주소 mac_주소` : 다른 서브넷의 IP를 기반으로 장치에 WoL 패킷 전송
- `wol -h 호스트명 mac_주소` : 다른 서브넷의 호스트명을 기반으로 장치에 WoL 패킷 전송
- `wol -p 포트_번호 mac_주소` : 특정 포트의 호스트에 WoL 패킷 전송
- `wol -f 경로/대상/파일` : 하드웨어 주소, IP 주소/호스트명, 선택적 포트 및 SecureON 비밀번호를 f파일에서 읽기
- `wol -v mac_주소` : v자세히 출력 켜기

## Links
- More information: https://sourceforge.net/projects/wake-on-lan/
