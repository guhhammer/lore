# adb shell

Android Debug Bridge Shell: Executar remotamente comandos shell em instâncias do emulador Android ou dispositivos Android conectados.

## Usage
- `adb shell` : inicia um shell interativo remoto no emulador/dispositivo
- `adb shell getprop` : obtém todas as propriedades do emulador ou dispositivo
- `adb shell pm reset-permissions` : reverte todas as permissões de tempo de execução para o padrão
- `adb shell pm revoke pacote permissao` : revoga uma permissão perigosa para um aplicação
- `adb shell input keyevent keycode` : aciona um evento
- `adb shell pm clear pacote` : limpa os dados da aplicação no emulador/dispositivo
- `adb shell am start -n pacote/atividade` : inicia uma atividade no emulator/dispositivo
- `adb shell am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : inicia atividade "home" no emulator/dispositivo

## Links
- More information: https://developer.android.com/tools/adb
