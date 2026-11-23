# mypy

Verificación de código Python.

## Usage
- `mypy ruta/al/archivo.py` : comprueba un archivo específico
- `mypy -m nombre_del_módulo` : comprueba un módulo específico
- `mypy -p nombre_del_paquete` : comprueba un paquete específico
- `mypy -c "código"` : comprueba una cadena de código
- `mypy --ignore-missing-imports ruta/al/archivo_o_directorio` : ignora importaciones faltantes
- `mypy --tb ruta/al/archivo_o_directorio` : muestra mensajes de error detallados
- `mypy --config-file ruta/al/archivo_de_configuración` : especifica un archivo de configuración personalizado
- `mypy -h` : muestra la ayuda

## Links
- More information: https://mypy.readthedocs.io/en/stable/running_mypy.html
