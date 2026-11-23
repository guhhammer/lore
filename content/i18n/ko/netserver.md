# netserver

네트워크 처리량을 측정하는 벤치마킹 애플리케이션 `netperf`의 서버 측 명령.

## Usage
- `netserver` : 기본 포트(12865)에서 서버 시작 및 백그라운드로 포크
- `netserver -D` : 포그라운드에서 서버 시작 및 포크하지 않음
- `netserver -p 포트` : 포트 지정
- `netserver -4|6` : IPv[4] 또는 IPv[6] 강제 설정

## Links
- More information: https://manned.org/netserver
