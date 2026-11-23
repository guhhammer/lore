# hcitool

Bluetooth 장치에 연결을 모니터링, 구성하고 특수 명령을 전송합니다.

## Usage
- `hcitool scan` : bluetooth 장치 검색
- `hcitool name bdaddr` : 장치의 이름을 출력하고 MAC 주소 반환
- `hcitool info bdaddr` : 원격 Bluetooth 장치 정보 가져오기
- `hcitool lq bdaddr` : bluetooth 장치와의 연결 품질 확인
- `hcitool tpl bdaddr 0|1` : 전송 전력 수준 수정
- `hcitool lp` : 연결 정책 표시
- `hcitool auth bdaddr` : 특정 장치와 인증 요청
- `hcitool dev` : 로컬 장치 표시

## Links
- More information: https://manned.org/hcitool
