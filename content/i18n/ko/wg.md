# wg

WireGuard 인터페이스의 구성을 관리합니다.

## Usage
- `sudo wg` : 현재 활성화된 인터페이스 상태 확인
- `wg genkey` : 새 개인 키 생성
- `wg <경로/대상/개인_키 pubkey >경로/대상/공개_키` : 개인 키로부터 공개 키 생성
- `wg genkey | tee 경로/대상/개인_키 | wg pubkey >경로/대상/공개_키` : 공개 키와 개인 키 생성
- `sudo wg showconf wg0` : wireGuard 인터페이스의 현재 구성 표시

## Links
- More information: https://www.wireguard.com/quickstart/
