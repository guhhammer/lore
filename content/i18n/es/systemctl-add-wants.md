# systemctl add-wants

Agrega dependencias `Wants` a un target para una o más unidades.

## Usage
- `systemctl add-wants target unidad` : agrega una dependencia `Wants` desde un target a una unidad
- `systemctl add-wants target unidad1 unidad2 ...` : agrega múltiples dependencias `Wants` a la vez
- `systemctl add-wants target unidad --user` : agrega una dependencia `Wants` a nivel de usuario
