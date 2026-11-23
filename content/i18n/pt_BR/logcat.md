# logcat

Despeja um registro de mensagens do sistema.

## Usage
- `logcat` : exibe a saída do registro
- `logcat -f caminho/para/arquivo` : salva a saída da mensagem de registro em um arquivo
- `logcat --regex expressao_regular` : exibe apenas linhas em que a mensagem de registro corresponda a uma expressão regular
- `logcat --pid pid` : exibe logs para um PID específico
- `logcat --pid $(pidof -s pacote)` : exibe logs de processo de um pacote específico

## Links
- More information: https://developer.android.com/tools/logcat
