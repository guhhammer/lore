# bitcoind

Daemon de Bitcoin Core. Utiliza la configuración definida en `bitcoin.conf`.

## Usage
- `bitcoind` : inicia el daemon Bitcoin Core (en primer plano)
- `bitcoind -daemon` : inicia el daemon Bitcoin Core en segundo plano (usa `bitcoin-cli stop` para detenerlo)
- `bitcoind -chain=main|test|signet|regtest` : inicia el daemon Bitcoin Core en una red específica
- `bitcoind -conf=ruta/a/bitcoin.conf -datadir=ruta/al/directorio` : inicia el daemon Bitcoin Core usando un archivo de configuración y directorio de datos específicos

## Links
- More information: https://manned.org/bitcoind
