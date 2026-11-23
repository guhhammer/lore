# gpclient

OpenConnect를 통해 Linux에서 GlobalProtect VPN에 연결.

## Usage
- `gpclient connect vpn_게이트웨이_url` : 포털 서버를 사용하여 GlobalProtect VPN에 연결
- `gpclient disconnect` : 현재 연결된 VPN 서버에서 연결 해제
- `gpclient launch-gui` : VPN 관리를 위한 그래픽 사용자 인터페이스(GUI) 실행
- `gpclient connect --fix-openssl vpn_게이트웨이_url` : openSSL 우회 방법을 사용하여 레거시 재협상 오류 우회
- `gpclient connect --ignore-tls-errors vpn_게이트웨이_url` : 연결 중 TLS 오류 무시
- `gpclient help 명령` : 명령에 대한 도움말 표시
- `gpclient --version` : 버전 표시

## Links
- More information: https://github.com/yuezk/GlobalProtect-openconnect
