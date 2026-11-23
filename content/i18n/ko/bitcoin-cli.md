# bitcoin-cli

RPC 호출을 통해 비트코인 데몬과 상호 작용하는 커맨드라인 클라이언트.`bitcoin.conf`에 정의된 구성 사용.

## Usage
- `bitcoin-cli sendtoaddress "주소" 양` : 지정된 주소로 트랜잭션 전송
- `bitcoin-cli generate 블록_갯수` : 하나 이상의 블록 생성
- `bitcoin-cli getwalletinfo` : wallet에 대한 고급 정보 출력
- `bitcoin-cli listunspent` : 보내지지 않은 모든 트랜잭션의 출력 나열
- `bitcoin-cli dumpwallet "경로/대상/파일"` : wallet 정보를 텍스트 파일로 출력
- `bitcoin-cli getblockchaininfo` : 블록체인 정보 출력
- `bitcoin-cli getnetworkinfo` : 네트워크 정보 출력
- `bitcoin-cli stop` : 비트코인 코어 데몬 중지

## Links
- More information: https://en.bitcoin.it/wiki/Running_Bitcoin#Command-line_arguments
