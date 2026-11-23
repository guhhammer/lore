# bitcoind

Bitcoin Core 데몬. `bitcoin.conf`에 정의된 구성을 사용.

## Usage
- `bitcoind` : 비트코인 코어 데몬을 시작 (포어그라운드 환경에서)
- `bitcoind -daemon` : 백그라운드에서 Bitcoin Core 데몬을 시작 (중지하려면 `bitcoin-cli stop` 사용)
- `bitcoind -chain=main|test|signet|regtest` : 특정 네트워크에서 Bitcoin Core 데몬을 시작
- `bitcoind -conf=경로/대상/bitcoin.conf -datadir=경로/대상/디렉터리` : 특정 구성 파일 및 데이터 디렉터리를 사용하여 Bitcoin Core 데몬을 시작

## Links
- More information: https://manned.org/bitcoind
