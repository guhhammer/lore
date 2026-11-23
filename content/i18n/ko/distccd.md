# distccd

distcc 분산 컴파일러용 서버 데몬.

## Usage
- `distccd --daemon` : 기본 설정으로 데몬을 시작
- `distccd --daemon --allow-private` : IPv4 개인 네트워크 범위 연결을 수락해 데몬을 시작
- `distccd --daemon -a ip_주소|네트워크_접두사` : 특정 네트워크 주소 또는 주소 버위로부터의 연결을 수락해 데몬을 시작
- `distccd --daemon -j 4 -N 5` : 한 번에 최대 4개의 작업을 실행할 수 있는 낮은 우선순위로 데몬을 시작
- `distccd --daemon --zeroconf` : 데몬을 시작하고 mDNS/DNS-SD (Zeroconf)를 통해 등록

## Links
- More information: https://manned.org/distccd
