# frps

Configura rápidamente un servicio de proxy inverso. Parte de `frp`.

## Usage
- `frps` : inicia el servicio, utilizando el archivo de configuración por defecto (se supone que es `frps.ini` en el directorio actual)
- `frps -c ruta/al/frps.toml` : inicia el servicio, utilizando el nuevo archivo de configuración TOML (`frps.toml` en lugar de `frps.ini`) en el directorio actual
- `frps -c ruta/al/archivo` : inicia el servicio, utilizando un archivo de configuración especificado
- `frps verify -c ruta/al/archivo` : comprueba si el archivo de configuración es válido
- `frps completion bash|fish|powershell|zsh` : imprime secuencia de comandos de configuración de autocompletado para Bash, fish, PowerShell o Zsh
- `frps -v` : muestra la versión

## Links
- More information: https://github.com/fatedier/frp
