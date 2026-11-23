# cryptcat

Cryptcat은 암호화 기능을 갖춘 netcat입니다.

## Usage
- `cryptcat -k 비밀번호 -l -p 포트` : 지정된 p포트에서 l수신하고 받은 데이터를 출력
- `cryptcat -k 비밀번호 ip_주소 포트` : 특정 포트에 연결
- `cryptcat -k 비밀번호 -w 초_단위_타임아웃 ip_주소 포트` : 타임아웃(w) 지정
- `cryptcat -v -z ip_주소 포트` : 지정된 호스트의 열린 포트 z스캔
- `cryptcat -k 비밀번호 -l -p 로컬_포트 | cryptcat -k 비밀번호 호스트명 원격_포트` : 프록시로 작동하여 로컬 TCP 포트에서 지정된 원격 호스트로 데이터 전달

## Links
- More information: https://manned.org/cryptcat
