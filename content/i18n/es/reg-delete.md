# reg delete

Elimina claves o sus valores del registro.

## Usage
- `reg delete nombre_clave` : elimina una clave de registro específica
- `reg delete nombre_clave /v valor` : elimina un valor bajo una clave específica
- `reg delete nombre_clave /va` : elimina todos a los valores recursivamente bajo la clave especificada
- `reg delete nombre_clave /f /va` : forza (sin un aviso) la eliminación de todos a los valores recursivamente bajo una clave
