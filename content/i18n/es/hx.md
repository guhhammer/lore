# hx

Helix, un editor de texto post-moderno, proporciona varios modos para diferentes tipos de manipulación de texto. Pulsando `<i>` se entra en el modo de inserción. `<Esc>` activa el modo normal, que permite el uso de los comandos de Helix.

## Usage
- `hx ruta/al/archivo` : abre un archivo
- `hx --vsplit ruta/al/archivo1 ruta/al/archivo2 ...` : abre archivos y los muestra uno al lado del otro
- `hx --tutor` : muestra el tutorial para aprender Helix (o accede a él dentro de Helix pulsando `<Esc>` y escribiendo `<:>tutor<Enter>`)
- `<:>theme nombre_tema` : cambia el tema de Helix
- `<:>wq<Enter>` : guarda y se cierra
- `<:>q!<Enter>` : fuerza la salida sin guardar
- `<u>` : deshace la última operación
- `</>patrón_de_búsqueda<Intro>` : busca un patrón en el archivo (pulsa `<n>`/`<N>` para ir a la siguiente/anterior coincidencia)

## Links
- More information: https://manned.org/man/debian-forky/hx
