# tc

트래픽 제어 설정을 표시/조작합니다.

## Usage
- `sudo tc q a dev eth0 root netem delay 지연_시간_밀리초ms` : 아웃바운드 패킷에 일정한 네트워크 지연 추가
- `sudo tc q a dev eth0 root netem delay 평균_지연_시간_밀리초ms 지연_표준_편차_밀리초ms` : 아웃바운드 패킷에 정규 분포된 네트워크 지연 추가
- `sudo tc q a dev eth0 root netem 손상|손실|중복 효과_비율%` : 일부 패킷에 손상/손실/중복 추가
- `sudo tc q a dev eth0 root tbf rate 최대_대역폭_메가비트mbit burst 최대_버스트_속도_킬로비트kbit latency 드롭_전_최대_지연_시간_밀리초ms` : 대역폭, 버스트 속도 및 최대 지연 시간 제한
- `tc q s dev eth0` : 활성 트래픽 제어 정책 표시
- `sudo tc q d dev eth0` : 모든 트래픽 제어 규칙 삭제
- `sudo tc q c dev eth0 root netem 정책 정책_매개변수` : 트래픽 제어 규칙 변경

## Links
- More information: https://manned.org/tc
