# pyrit

계산 능력을 활용한 WPA/WPA2 크래킹 도구.

## Usage
- `pyrit benchmark` : 시스템 크래킹 속도 표시
- `pyrit list_cores` : 사용 가능한 코어 나열
- `pyrit -e "extended_ssid" create_essid` : eSSID 설정
- `pyrit -r 경로/대상/파일.cap|pcap analyze` : 특정 패킷 캡처 파일 r읽고 분석
- `pyrit -i 경로/대상/파일 import_unique_passwords|unique_passwords|import_passwords` : 현재 데이터베이스에 비밀번호 i가져오기
- `pyrit -o 경로/대상/파일 export_passwords` : 데이터베이스에서 특정 파일로 비밀번호 o내보내기
- `pyrit batch` : pired 마스터 키로 비밀번호 변환
- `pyrit -r 경로/대상/파일 attack_db` : 캡처 파일 r읽고 비밀번호 크래킹

## Links
- More information: https://manned.org/pyrit
