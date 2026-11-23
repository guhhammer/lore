# electrum

인체공학적 Bitcoin 지갑 및 개인 키 관리.

## Usage
- `electrum -w 경로/대상/new_wallet.dat create` : 새로운 지갑 생성
- `electrum -w 경로/대상/recovery_wallet.dat restore -o` : 오프라인 시드에서 기존 지갑 복원
- `electrum mktx recipient amount -f 0.0000001 -F from -o` : 오프라인으로 서명된 거래 생성
- `electrum listaddresses -a` : 모든 지갑 수신 주소 표시
- `electrum signmessage 주소 메시지` : 메시지에 설명
- `electrum verifymessage 주소 서명 메시지` : 메시지 확인
- `electrum -p socks5:127.0.0.1:9050 -s 56ckl5obj37gypcu.onion:50001:t -1` : 특정 일렉트럼 서버 인스턴스에만 연결

## Links
- More information: https://manned.org/electrum
