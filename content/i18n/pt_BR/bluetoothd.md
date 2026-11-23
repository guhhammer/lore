# bluetoothd

Daemon para gerenciar dispositivos Bluetooth.

## Usage
- `bluetoothd` : inicia o daemon
- `bluetoothd -n` : inicia o daemon, registrando em `stdout`
- `bluetoothd -f caminho/para/arquivo` : inicia o daemon com um arquivo de configuração específico (`/etc/bluetooth/main.conf` por padrão)
- `bluetoothd -d` : inicia o daemon com saída verbosa em `stderr`
- `bluetoothd -dcaminho/para/arquivo1:caminho/para/arquivo2:...` : inicia o daemon com saída verbosa proveniente de arquivos específicos na fonte bluetoothd ou plugins

## Links
- More information: https://manned.org/bluetoothd
