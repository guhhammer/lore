# avahi-browse

로컬 네트워크에서 mDNS/DNS-SD를 통해 노출된 서비스와 호스트를 표시합니다. Avahi는 Apple 기기에서 발견되는 Bonjour(Zeroconf)와 호환됩니다.

## Usage
- `avahi-browse -a -r -l` : 로컬 네트워크에서 사용 가능한 서비스와 해당 주소 및 포트를 나열하되, 로컬 머신의 서비스는 무시
- `avahi-browse -a -t -p` : 스크립트를 위한 SSV 형식으로 로컬 네트워크의 서비스를 빠르게 나열
- `avahi-browse -D` : 주변 도메인 나열
- `avahi-browse -a --domain=도메인` : 특정 도메인으로 검색 제한

## Links
- More information: https://manned.org/avahi-browse
