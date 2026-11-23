# wezterm

Wez's Terminal Emulator - un potente emulador de terminal multiplataforma y multiplexor.

## Usage
- `wezterm` : inicia un nuevo proceso Wezterm y crea una ventana
- `wezterm ssh usuario@host:puerto` : establece una sesión `ssh`
- `wezterm connect nombre_dominio` : conecta con el multiplexor (`wezterm-mux-server`)
- `wezterm imgcat ruta/a/imagen` : envía una imagen al terminal
- `wezterm record` : graba una sesión de terminal como un asciicat (por defecto las grabaciones se encuentran en `/tmp`)
- `wezterm replay ruta/al/archivo_cast` : reproduce una sesión de terminal asciicat
- `wezterm --config-file ruta/al/archivo_config` : especifica el archivo de configuración a utilizar (anula la resolución normal del archivo de configuración)
- `wezterm help` : muestra la ayuda

## Links
- More information: https://wezterm.org/cli/general
