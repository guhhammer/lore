# adb logcat

Extrai um registro (log) de mensagens do sistema.

## Usage
- `adb logcat` : exibe registros (logs) do sistema
- `adb logcat -e regex` : exibe linhas que correspondem a uma `regex`
- `adb logcat tag:modo *:S` : exibe registros de uma tag em um modo específico (Verbose (detalhado), Debug (depuração), Info (informação), Warning (aviso), Error (erro), Fatal (fatal), Silent (silencioso)), filtrando outras tags
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : exibe registros de aplicações React Native em modo Verboso, silenciando outras tags
- `adb logcat *:W` : exibe registros de todas as tags com nível de prioridade Warning (aviso) ou superior
- `adb logcat --pid pid` : exibe registros de um PID específico
- `adb logcat --pid $(adb shell pidof -s pacote)` : exibe registros do processo de um pacote específico
- `adb logcat -v color` : colore o registro (geralmente usado com filtros)

## Links
- More information: https://developer.android.com/tools/logcat
