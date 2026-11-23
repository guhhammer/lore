# uwfmgr

Unified Write Filter (UWF). Protege las unidades redirigiendo cualquier escritura a una superposición virtual. Las escrituras se descartan al reiniciar, a menos que se confirmen por defecto.

## Usage
- `uwfmgr get-config` : obtiene el estado actual
- `uwfmgr volume protect letra_de_unidad:` : establece una unidad como protegida
- `uwfmgr volume unprotect letra_de_unidad:` : quita una unidad de la lista de protección
- `uwfmgr filter enable|disable` : habilita o deshabilita la protección (se aplica después del reinicio)
- `uwfmgr file commit letra_de_unidad:\ruta\al\archivo` : confirma los cambios de un archivo en una unidad protegida
- `uwfmgr file commit-delete letra_de_unidad:\ruta\al\archivo` : confirma la eliminación de un archivo en una unidad protegida
