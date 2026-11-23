# bitcoind

Bitcoin Core daemon. Uses the configuration defined in `bitcoin.conf`.

## Usage
- `bitcoind` : start the Bitcoin Core daemon (in the foreground)
- `bitcoind -daemon` : start the Bitcoin Core daemon in the background (use `bitcoin-cli stop` to stop)
- `bitcoind -chain=main|test|signet|regtest` : start the Bitcoin Core daemon on a specific network
- `bitcoind -conf=path/to/bitcoin.conf -datadir=path/to/directory` : start the Bitcoin Core daemon using specific configuration file and data directory

## Categories
system, files, network

## Links
- More information: https://manned.org/bitcoind
- tldr page: https://tldr.inbrowser.app/pages/common/bitcoind
