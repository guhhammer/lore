# reg add

Agrega nuevas claves y sus valores al registro.

## Usage
- `reg add nombre_clave` : agrega una nueva clave de registro
- `reg add nombre_clave /v valor` : agrega un nuevo valor bajo una clave específica
- `reg add nombre_clave /d datos` : agrega un nuevo valor con datos específicos
- `reg add nombre_clave /t REG_SZ|MULTI_SZ|DWORD_BIG_ENDIAN|DWORD|BINARY|DWORD_LITTLE_ENDIAN|LINK|FULL_RESOURCE_DESCRIPTOR|EXPAND_SZ` : agrega un nuevo valor a una clave con un tipo de dato específico
- `reg add nombre_clave /f` : fuerza (sin un aviso) la sobrescritura del valor de registro existente
