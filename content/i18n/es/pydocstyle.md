# pydocstyle

Comprueba estáticamente que los scripts de Python cumplen con las convenciones de documentación de Python.

## Usage
- `pydocstyle archivo.py|ruta/al/directorio` : analiza un script de Python o todos los scripts de Python en un directorio específico
- `pydocstyle -e archivo.py|ruta/al/directorio` : muestra una explicación de cada error
- `pydocstyle -d archivo.py|ruta/al/directorio` : muestra información de depuración
- `pydocstyle --count archivo.py|ruta/al/directorio` : muestra el número total de errores
- `pydocstyle --config ruta/al/archivo_config archivo.py|ruta/al/directorio` : utiliza un archivo de configuración específico
- `pydocstyle --ignore D101,D2,D107,... archivo.py|ruta/al/directorio` : ignora uno o más errores
- `pydocstyle --convention pep257|numpy|google archivo.py|ruta/al/directorio` : busca errores de una convención específica

## Links
- More information: https://pydocstyle.readthedocs.io/en/latest/usage.html
