# openfortivpn

Fortinet의 독점 PPP+SSL VPN 솔루션을 위한 VPN 클라이언트.

## Usage
- `openfortivpn -u 사용자명 -p 비밀번호` : 사용자명과 비밀번호로 VPN에 연결
- `sudo openfortivpn -c 경로/대상/구성` : 특정 구성 파일을 사용하여 VPN에 연결 (`/etc/openfortivpn/config`가 기본값)
- `openfortivpn 호스트:포트` : 호스트와 포트를 지정하여 VPN에 연결
- `openfortivpn --trusted-cert sha256_합계` : 인증서의 sha256 합계를 전달하여 특정 게이트웨이를 신뢰

## Links
- More information: https://manned.org/openfortivpn
