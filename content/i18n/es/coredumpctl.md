# coredumpctl

Recupera y procesa volcados de memoria y sus metadatos.

## Usage
- `coredumpctl list` : lista todos los volcados de memoria capturados
- `coredumpctl list programa` : lista los volcados de memoria capturados para un programa
- `coredumpctl info pid` : muestra información sobre los volcados de memoria que coincidan con el `PID` de un programa
- `coredumpctl debug programa` : invoca el depurador usando el último volcado de memoria para un programa
- `coredumpctl --output ruta/al/archivo dump programa` : extrae el último volcado de memoria a un fichero

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/coredumpctl.html
