# new-item

Crea un nuevo archivo, directorio, enlace simbólico o una entrada de registro. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `New-Item ruta\al\archivo` : crea un nuevo archivo en blanco (equivalente a `touch`)
- `New-Item -ItemType Directory ruta\al\directorio` : crea un nuevo directorio
- `New-Item ruta\al\archivo -Value contenido` : escribe un nuevo archivo de texto con el contenido especificado
- `New-Item ruta\al\archivo1 , ruta\al\archivo2 , ... -Value contenido` : escribe el mismo archivo de texto en múltiples ubicaciones
- `New-Item -ItemType SymbolicLink|HardLink|Junction -Path ruta\al\archivo_enlace -Target ruta\al\archivo_o_directorio_fuente` : crea un enlace simbólico\enlace duro\unión a un archivo o directorio
- `New-Item ruta\a\la\clave_de_registro` : crea una nueva entrada de registro en blanco (en REG_SZ, usar `New-ItemProperty` o `Set-ItemProperty` para ajustar el tipo de valor)
- `New-Item ruta\a\la\clave_de_registro -Value valor` : crea una nueva entrada de registro en blanco con un valor especificado
