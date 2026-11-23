# if

Realiza procesamiento condicional en scripts por lotes.

## Usage
- `if condición (echo La condición es verdadera)` : ejecuta los comandos especificados si la condición es verdadera
- `if not condición (echo La condición es verdadera)` : ejecuta los comandos especificados si la condición es falsa
- `if condición (echo La condición es verdadera) else (echo La condición es falsa)` : ejecuta los primeros comandos especificados si la condición es verdadera, de lo contrario, ejecuta los segundos comandos especificados
- `if errorlevel 2 (echo La condición es verdadera)` : verifica si `%errorlevel%` es mayor o igual al código de salida especificado
- `if %variable% == cadena (echo La condición es verdadera)` : verifica si dos cadenas son iguales
- `if /i %variable% == cadena (echo La condición es verdadera)` : verifica si dos cadenas son iguales sin respetar el caso de las letras
- `if exist ruta\al\archivo (echo La condición es verdadera)` : verifica si un archivo existe
