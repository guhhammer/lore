# dumpsys

Fornece informações sobre os serviços do sistema Android. Este comando só pode ser usado através de `adb shell`.

## Usage
- `dumpsys` : gera um diagnóstico de todos os serviços do sistema
- `dumpsys servico` : gera um diagnóstico de um serviço do sistema específico
- `dumpsys -l` : lista todos os serviços que o `dumpsys` pode obter informações
- `dumpsys servico -h` : lista argumentos específicos-de-um-serviço para um serviço
- `dumpsys --skip servico` : omite um serviço em específico do diagnóstico
- `dumpsys -t segundos` : específica um periodo de timeout (por padrão é 10s)

## Links
- More information: https://developer.android.com/tools/dumpsys
