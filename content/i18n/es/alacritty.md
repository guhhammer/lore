# alacritty

Emulador de terminal multiplataforma acelerado por GPU.

## Usage
- `alacritty` : inicia un nuevo proceso Alacritty y crea una ventana
- `alacritty --daemon` : inicia el programa residente de Alacritty (sin crear una ventana)
- `alacritty msg create-window` : crea una nueva ventana utilizando el proceso Alacritty ya en ejecución
- `alacritty --working-directory ruta/al/directorio` : inicia el intérprete de comandos en un directorio específico (también funciona con `alacritty msg create-window`)
- `alacritty -e comando` : ejecuta un comando en una nueva ventana de Alacritty (también funciona con `alacritty msg create-window`)
- `alacritty --config-file ruta/a/config.toml` : utiliza un archivo de configuración alternativo (por defecto es `$XDG_CONFIG_HOME/alacritty/alacritty.toml`)

## Links
- More information: https://manned.org/alacritty
