# fastd

VPN 데몬. 레이어 2 또는 레이어 3에서 작동하며, Freifunk에서 사용하는 다양한 암호화 방법을 지원.

## Usage
- `fastd -c 경로/대상/fastd.conf` : 특정 구성 파일로 `fastd`를 시작
- `fastd -m tap -M 1400 -c 경로/대상/fastd.conf` : MTU 1400으로 레이어 3 VPN을 시작하고 파일에서 나머지 구성 매개변수를 로드
- `fastd --verify-config -c 경로/대상/fastd.conf` : 구성 파일의 유효성을 검증
- `fastd --generate-key` : 새로운 키의 쌍을 생성
- `fastd --show-key -c 경로/대상/fastd.conf` : 구성 파일의 개인 키에 공개 키를 표시
- `fastd -v` : 버전 표시

## Links
- More information: https://fastd.readthedocs.io/en/stable/
