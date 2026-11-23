# iperf

컴퓨터 간 네트워크 대역폭을 측정하는 도구.

## Usage
- `iperf -s` : 서버 모드로 실행
- `iperf -u -s -p 5001` : UDP 모드로 서버 실행 및 포트 5001에서 대기
- `iperf -c 서버_주소` : 클라이언트 모드로 실행
- `iperf -c 서버_주소 -i 2` : 2초 간격으로 클라이언트 실행
- `iperf -c 서버_주소 -P 5` : 5개의 병렬 스레드로 클라이언트 실행
- `iperf -u -c 서버_주소 -p 5001` : UDP 모드로 클라이언트 실행

## Links
- More information: https://iperf.fr/iperf-doc.php
