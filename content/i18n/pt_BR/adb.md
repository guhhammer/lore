# adb

Android Debug Bridge: Comunica com uma instância do emulador Android emulator ou dispositivos conectados. Alguns subcomandos tais como `shell` possuem sua própria documentação de uso.

## Usage
- `adb start-server` : checa se o servidor adb está em execução e o inicia
- `adb kill-server` : encerra o processo do servidor adb
- `adb shell` : inicia uma shell remota no emulador/dispositivo desejado
- `adb install -r caminho/para/arquivo.apk` : instala um app Android no emulador/dispositivo
- `adb pull caminho/para/arquivo_ou_pasta_no_dispositivo caminho/para/pasta_de_destino_local` : copia um arquivo/pasta do dispositivo desejado
- `adb push caminho/para/arquivo_ou_pasta_local caminho/para/pasta_de_destino_no_dispositivo` : copia um arquivo/pasta para o dispositivo desejado
- `adb devices` : exibe a lista de dispositivos conectados

## Links
- More information: https://developer.android.com/tools/adb
