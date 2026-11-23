# mas

Interfaz de línea de comandos para la Mac App Store.

## Usage
- `mas signin "usuario@example.com"` : inicia sesión en la Mac App Store por primera vez
- `mas list` : muestra todas las aplicaciones instaladas y sus identificadores de producto
- `mas search "aplicación" --price` : busca una aplicación, mostrando el precio junto a los resultados
- `mas install id_producto_numérico` : instala o actualiza una aplicación utilizando el identificador numérico exacto
- `mas lucky "término_de_búsqueda"` : instala la primera aplicación que devuelva la búsqueda correspondiente
- `mas outdated` : lista todas las aplicaciones obsoletas con actualizaciones pendientes
- `mas upgrade` : instala todas las actualizaciones pendientes
- `mas upgrade "id_producto_numérico"` : actualiza una aplicación específica

## Links
- More information: https://github.com/mas-cli/mas
