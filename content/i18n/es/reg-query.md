# reg query

Muestra los valores de claves y subclaves en el registro.

## Usage
- `reg query nombre_de_clave` : muestra todos los valores de una clave
- `reg query nombre_de_clave /v valor` : muestra un valor específico de una clave
- `reg query nombre_de_clave /s` : muestra todos los valores de una clave y sus subclaves
- `reg query nombre_de_clave /f "patrón_de_búsqueda"` : busca f claves y valores que coincidan con un patrón específico
- `reg query nombre_de_clave /t REG_SZ|MULTI_SZ|EXPAND_SZ|DWORD|BINARY|NONE` : muestra un valor de una clave que coincida con un tipo de dato específico
- `reg query nombre_de_clave /d` : busca solo en los datos
- `reg query nombre_de_clave /f "patrón_de_búsqueda" /k` : busca solo en los nombres de clave k
- `reg query nombre_de_clave /c /e` : busca una coincidencia exacta distinguiendo entre mayúsculas y minúsculas c
