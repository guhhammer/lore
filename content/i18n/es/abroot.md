# abroot

Utilidad que proporciona completa inmutabilidad y atomicidad mediante transacciones entre 2 estados de partición de la raíz (A<=>B). Las actualizaciones se realizan utilizando imágenes OCI, para asegurar que el sistema está siempre en un estado consistente.

## Usage
- `sudo abroot pkg add paquete` : añade paquetes a la imagen local (Nota: después de ejecutar este comando, se necesita aplicar estos cambios)
- `sudo abroot pkg remove paquete` : elimina paquetes de la imagen local (Nota: después de ejecutar este comando, debe aplicar estos cambios)
- `sudo abroot pkg list` : lista paquetes en la imagen local
- `sudo abroot pkg apply` : aplica los cambios en la imagen local (Nota: es necesario reiniciar el sistema para que estos cambios sean aplicados)
- `sudo abroot rollback` : retrocede su sistema al estado anterior
- `sudo abroot kargs edit|show` : edita/Visualiza los parámetros del kernel
- `sudo abroot status` : muestra estado
- `abroot -h` : muestra la ayuda

## Links
- More information: https://docs.vanillaos.org/docs/en/abroot-manpage
