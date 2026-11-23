# uv add

Añade dependencias de paquetes al archivo `pyproject.toml`.

## Usage
- `uv add paquete` : añade la última versión de un paquete
- `uv add paquete1 paquete2 ...` : añade múltiples paquetes
- `uv add paquete>=1.2.3` : añade un paquete con un requisito de versión
- `uv add --optional opcional paquete1 paquete2 ...` : añade paquetes a un grupo de dependencia opcional, que se incluirá cuando se publique
- `uv add --group grupo paquete1 paquete2 ...` : añade paquetes a un grupo local, que no se incluirán cuando se publiquen
- `uv add --dev paquete1 paquete2 ...` : añade paquetes al grupo dev, abreviatura de `--group dev`
- `uv add --editable ruta/a/paquete/` : añade paquete como editable
- `uv add paquete --extra característica_extra` : habilita un extra al instalar el paquete, se puede proporcionar varias veces

## Links
- More information: https://peps.python.org/pep-0508/
