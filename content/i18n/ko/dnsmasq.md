# dnsmasq

경량 DNS, DHCP, TFTP, PXE 서버.

## Usage
- `dnsmasq` : 기본 설정으로 dnsmasq 시작
- `dnsmasq --no-daemon` : 포그라운드에서 dnsmasq 실행 (디버깅용)
- `dnsmasq --conf-file=경로/대상/설정파일.conf` : 사용자 정의 설정 파일 지정
- `dnsmasq --log-queries --log-facility=-` : 상세 로그 출력 활성화
- `dnsmasq --dhcp-range=192.168.0.50,192.168.0.150,12h` : DHCP 범위 및 임대 시간 설정
- `dnsmasq --version` : 버전 표시

## Links
- More information: https://manned.org/dnsmasq
