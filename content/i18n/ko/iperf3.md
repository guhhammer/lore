# iperf3

네트워크 대역폭 테스트를 위한 트래픽 생성기입니다.

## Usage
- `iperf3 -s` : iperf3를 서버로 실행
- `iperf3 -s -p 포트` : 특정 포트에서 iperf3 서버 실행
- `iperf3 -c 서버` : 대역폭 테스트 시작
- `iperf3 -c 서버 -P 스트림` : 여러 병렬 스트림에서 iperf3 실행
- `iperf3 -c 서버 -R` : 테스트를 역방향으로 진행합니다. 서버가 클라이언트에 데이터를 전송

## Links
- More information: https://iperf.fr/iperf-doc.php
