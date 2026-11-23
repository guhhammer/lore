# wsl

Administra el Subsistema de Windows para Linux.

## Usage
- `wsl comando_de_shell` : inicia un intérprete de comandos de Linux (usando la distribución predeterminada)
- `wsl -e comando argumentos_del_comando` : ejecuta un comando de Linux sin usar un intérprete de comandos
- `wsl -d distribución comando_de_shell` : especifica una distribución particular
- `wsl -l` : lista las distribuciones disponibles
- `wsl --export distribución ruta\a\archivo_de_distribucion.tar` : exporta una distribución a un archivo `.tar`
- `wsl --import distribución ruta\a\ubicacion_de_instalacion ruta\a\archivo_de_distribucion.tar` : importa una distribución de un archivo `.tar`
- `wsl --set-version distribución versión` : cambia la versión de wsl usada para la distribución especificada
- `wsl --shutdown` : apaga el Subsistema de Windows para Linux

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
