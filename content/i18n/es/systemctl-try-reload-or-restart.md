# systemctl try-reload-or-restart

Recarga una o más unidades si lo admiten; de lo contrario, las reinicia.

## Usage
- `systemctl try-reload-or-restart unidad` : recarga o reinicia una unidad específica
- `systemctl try-reload-or-restart unidad1 unidad2 ...` : recarga o reinicia varias unidades
- `systemctl try-reload-or-restart 'patrón'` : recarga o reinicia todas las unidades que coincidan con un patrón
