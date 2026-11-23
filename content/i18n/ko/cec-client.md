# cec-client

직렬 버스 CEC 연결 관리.

## Usage
- `cec-client -l` : 모든 CEC 어댑터 목록 표시
- `sudo cec-client` : 대화형 CEC 세션 시작
- `sudo cec-client -o 이름` : on-Screen Display 이름 설정
- `echo on 0 | sudo cec-client -s` : 단일 명령 전송
- `standby 0` : [대화형] 대화형 모드에서 장치를 대기(standby) 상태로 전환
- `on 0` : [대화형] 대화형 모드에서 장치 전원 켜기

## Links
- More information: https://manned.org/cec-client
