# warp-cli

Cloudflare의 WARP 서비스에 대한 연결을 연결, 연결 해제하고 모드를 전환. WARP는 개인정보 보호, 보안, 속도를 위해 트래픽을 암호화하는 VPN입니다.

## Usage
- `warp-cli registration new` : 현재 장치를 WARP에 등록 (첫 연결 전에 실행 필요)
- `warp-cli connect` : WARP에 연결
- `warp-cli disconnect` : WARP에서 연결 해제
- `warp-cli status` : WARP 연결 상태 표시
- `warp-cli mode warp|doh|warp+doh|dot|warp+dot|proxy|tunnel_only` : 특정 모드로 전환

## Links
- More information: https://developers.cloudflare.com/warp-client/
