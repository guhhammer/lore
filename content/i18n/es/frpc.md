# frpc

Se conecta a un servidor `frps` para iniciar conexiones proxy en el host actual. Parte de `frp`.

## Usage
- `frpc` : inicia el servicio, utilizando el archivo de configuración por defecto (se supone que es `frps.ini` en el directorio actual)
- `frpc -c ruta/al/frpc.toml` : inicia el servicio, utilizando el nuevo archivo de configuración TOML (`frps.toml` en lugar de `frps.ini`) en el directorio actual
- `frpc -c ruta/al/archivo` : inicia el servicio, utilizando un archivo de configuración específico
- `frpc verify -c ruta/al/archivo` : comprueba si el archivo de configuración es válido
- `frpc completion bash|fish|powershell|zsh` : imprime secuencia de comandos de configuración de autocompletado para Bash, fish, PowerShell o Zsh
- `frpc -v` : muestra la versión

## Links
- More information: https://github.com/fatedier/frp
