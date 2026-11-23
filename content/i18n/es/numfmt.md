# numfmt

Convierte números a y desde cadenas legibles por humanos.

## Usage
- `numfmt --from si 1.5K` : convierte 1.5K (Unidades SI) a 1500
- `ls -l | numfmt --header=1 --field 5 --to iec` : convierte el 5º campo (1-indexado) a Unidades IEC sin convertir la cabecera
- `du -s * | numfmt --to iec --format "%-5f"` : convierte a unidades IEC, los rellena con 5 caracteres, alineado a la izquierda
