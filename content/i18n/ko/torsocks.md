# torsocks

모든 애플리케이션의 트래픽을 Tor 네트워크를 통해 라우팅합니다.

## Usage
- `torsocks 명령` : tor를 사용하여 명령 실행
- `. torsocks on|off` : 이 셸에서 Tor 활성화 또는 비활성화
- `torsocks --shell` : tor가 활성화된 새로운 셸 생성
- `torsocks show` : 현재 셸이 Tor가 활성화되었는지 확인 (`$LD_PRELOAD` 값이 비어 있으면 비활성화됨)
- `torsocks -i curl https://check.torproject.org/api/ip` : 다른 Tor 회로를 통해 트래픽을 솔레이트하여 익명성 향상
- `torsocks -a ip_address -P 포트 명령` : 특정 주소 및 포트에서 실행 중인 Tor 프록시에 연결

## Links
- More information: https://manned.org/torsocks
