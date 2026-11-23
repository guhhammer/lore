# systemctl try-restart

Reinicia una o más unidades solo si están actualmente en ejecución.

## Usage
- `systemctl try-restart unidad` : reinicia una unidad específica si está en ejecución
- `systemctl try-restart unidad1 unidad2 ...` : reinicia varias unidades si están en ejecución
- `systemctl try-restart 'patrón'` : reinicia todas las unidades que coincidan con un patrón si están en ejecución
