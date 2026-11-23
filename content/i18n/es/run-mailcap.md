# run-mailcap

Ejecuta programas mediante las entradas del archivo `mailcap`.

## Usage
- `run-mailcap --action=compose ruta/al/archivo` : edita cualquier archivo existente o crea uno nuevo con la herramienta de edición predeterminada de `mailcap`
- `run-mailcap --action=edit ruta/al/archivo` : muestra cualquier archivo en el explorador predeterminado de `mailcap`
- `run-mailcap --action=print ruta/al/archivo` : imprime un archivo utilizando la herramienta de impresión predeterminada de `mailcap`
- `run-mailcap --action=view ruta/al/archivo` : muestra cualquier archivo (normalmente una imagen) en el explorador predeterminado de `mailcap`
- `run-mailcap --action=view|cat|compose|composetyped|edit|print ruta/al/archivo` : ejecuta acciones o programas individuales en `run-mailcap`
- `run-mailcap --action=action --debug ruta/al/archivo` : activa información adicional
- `run-mailcap --action=action --nopager ruta/al/archivo` : ignora cualquier directiva «copiousoutput» y redirigir la salida a `stdout`
- `run-mailcap --action=action --norun ruta/al/archivo` : muestra el comando encontrado sin ejecutarlo realmente

## Links
- More information: https://manned.org/run-mailcap
