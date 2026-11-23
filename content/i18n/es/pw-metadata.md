# pw-metadata

Supervisa, establece y elimina metadatos en objetos PipeWire.

## Usage
- `pw-metadata` : muestra metadatos en el formato por defecto
- `pw-metadata -n settings 0` : muestra metadatos con el identificador 0 en `settings`
- `pw-metadata -l` : lista todos los objetos de metadatos disponibles
- `pw-metadata -m` : continua ejecutando y registrando los cambios en los metadatos
- `pw-metadata -d` : elimina todos los metadatos
- `pw-metadata -n settings 0 log.level 1` : ajusta `log.level` a 1 en `settings`
- `pw-metadata -h` : muestra la ayuda

## Links
- More information: https://docs.pipewire.org/page_man_pw-metadata_1.html
