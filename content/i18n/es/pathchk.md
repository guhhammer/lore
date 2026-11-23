# pathchk

Comprueba la validez y la portabilidad de las rutas de acceso.

## Usage
- `pathchk ruta1 ruta2 ...` : comprueba la validez de las rutas de acceso en el sistema actual
- `pathchk -p ruta1 ruta2 ...` : comprueba la validez de las rutas de acceso en una gama más amplia de sistemas compatibles con POSIX
- `pathchk -p -P ruta1 ruta2 ...` : comprueba la validez de las rutas en todos los sistemas compatibles con POSIX
- `pathchk -P ruta1 ruta2 ...` : comprueba únicamente si las rutas están vacías o comienzan por un guión (-)
