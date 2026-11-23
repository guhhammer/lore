# systemctl freeze

Congela una o más unidades. Las unidades congeladas se pueden reanudar con `systemctl thaw`.

## Usage
- `systemctl freeze unidad` : congela una unidad específica
- `systemctl freeze unidad1 unidad2 ...` : congela múltiples unidades
- `systemctl freeze '*'` : congela todas las unidades en ejecución
