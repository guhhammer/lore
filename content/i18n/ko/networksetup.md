# networksetup

네트워크 시스템 환경설정 구성 도구.

## Usage
- `networksetup -listallnetworkservices` : 사용 가능한 네트워크 서비스 제공자 나열 (이더넷, Wi-Fi, 블루투스 등)
- `networksetup -getinfo "Wi-Fi"` : 특정 네트워크 장치의 네트워크 설정 표시
- `networksetup -getairportnetwork en0` : 현재 연결된 Wi-Fi 네트워크 이름 가져오기 (Wi-Fi 장치는 보통 en0 또는 en1)
- `networksetup -setairportnetwork en0 ssid 비밀번호` : 특정 Wi-Fi 네트워크에 연결
