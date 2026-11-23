# bluebuild

Construye Containerfiles e imágenes personalizadas basadas en tu `recipe.yml`.

## Usage
- `bluebuild build ruta/a/receta.yml` : construye una receta
- `bluebuild validate ruta/a/receta.yml` : valida una receta
- `bluebuild generate -o archivo_contenedor ruta/a/receta.yml` : genera un archivo contenedor
- `bluebuild generate-iso --output-dir ruta/al/directorio_salida --iso-name nombre_iso.iso recipe ruta/a/receta.yml` : genera una ISO a partir de una receta
- `bluebuild -h` : muestra la ayuda

## Links
- More information: https://github.com/blue-build/cli#how-to-use
