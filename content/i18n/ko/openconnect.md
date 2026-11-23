# openconnect

Cisco AnyConnect VPN 및 기타 VPN을 위한 VPN 클라이언트.

## Usage
- `openconnect vpn.example.org` : 서버에 연결
- `openconnect --background vpn.example.org` : 백그라운드에서 실행되도록 서버에 연결
- `killall -SIGINT openconnect` : 백그라운드에서 실행 중인 연결 종료
- `openconnect --config=경로/대상/파일 vpn.example.org` : 설정 파일에서 옵션을 읽어 서버에 연결
- `openconnect --certificate=경로/대상/파일 vpn.example.org` : 특정 SSL 클라이언트 인증서로 인증하여 서버에 연결

## Links
- More information: https://www.infradead.org/openconnect/manual.html
