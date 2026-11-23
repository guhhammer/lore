# f5fpc

BIG-IP Edge의 독점 상업용 SSL VPN 클라이언트.

## Usage
- `sudo f5fpc --start` : 새 VPN 연결 열기
- `sudo f5fpc --start --host host.example.com` : 특정 호스트에 새 VPN 연결 열기
- `sudo f5fpc --start --host host.example.com --username 사용자` : 사용자명 지정 (암호는 사용자에게 요청됨)
- `sudo f5fpc --info` : 현재 VPN 상태 표시
- `sudo f5fpc --stop` : VPN 연결 종료

## Links
- More information: https://my.f5.com/manage/s/article/K47922841
