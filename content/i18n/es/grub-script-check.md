# grub-script-check

El programa `grub-script-check` toma un archivo de script de GRUB y lo verifica en busca de errores de sintaxis. Puede tomar una ruta como argumento no opcional. Si no se proporciona ninguna, lee desde `stdin`.

## Usage
- `grub-script-check ruta/al/archivo_config_grub` : verifica un archivo de script específico en busca de errores de sintaxis
- `grub-script-check -v` : muestra cada línea de entrada después de leerla
- `grub-script-check --help` : muestra la ayuda
- `grub-script-check --version` : muestra la versión
