# systemctl add-requires

Agrega dependencias `Requires` a un target para una o más unidades.

## Usage
- `systemctl add-requires target unidad` : agrega una dependencia `Requires` desde un target a una unidad
- `systemctl add-requires target unidad1 unidad2 ...` : agrega múltiples dependencias `Requires` a la vez
- `systemctl add-requires target unidad --user` : agrega una dependencia `Requires` a nivel de usuario
