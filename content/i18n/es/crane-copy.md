# crane copy

Copia eficientemente una imagen remota de origen a destino conservando el valor de resumen.

## Usage
- `crane cp origen destino` : copia una imagen de origen a destino
- `crane cp origen destino -a` : copia todas las etiquetas
- `crane cp origen destino -j número` : establece el número máximo de copias simultáneas, predeterminados a GOMAXPROCS
- `crane cp origen destino -n` : evita sobrescribir las etiquetas existentes en el destino
- `crane cp -h` : muestra la ayuda
