# ag

El Buscador de Plata. Como `ack`, pero aspira a ser más rápido.

## Usage
- `ag foo` : encuentra archivos que contengan "foo", e imprime las líneas coincidentes en contexto
- `ag foo ruta/al/directorio` : encuentra archivos que contengan "foo" en un directorio específico
- `ag -l foo` : encuentra archivos que contengan "foo", pero solo lista los nombres de archivo
- `ag -i -o FOO` : encuentra archivos que contengan "FOO" sin distinción entre mayúsculas y minúsculas, e imprime solo la coincidencia, en lugar de la línea completa
- `ag foo -G bar` : busca "foo" en archivos cuyo nombre coincida con "bar"
- `ag '^ba(r|z)$'` : busca archivos cuyo contenido coincida con una `regex`
- `ag -g foo` : busca archivos cuyo nombre coincida con "foo"

## Links
- More information: https://manned.org/ag
