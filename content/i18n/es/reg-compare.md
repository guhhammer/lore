# reg compare

Compara claves y sus valores en el registro.

## Usage
- `reg compare nombre_clave1 nombre_clave2` : compara todos los valores bajo una clave específica con otra clave
- `reg compare nombre_clave1 nombre_clave2 /v valor` : compara un valor específico bajo dos claves
- `reg compare nombre_clave1 nombre_clave2 /s` : compara todos los subclaves y valores para dos claves
- `reg compare nombre_clave1 nombre_clave2 /os` : solo genera un resultado con las coincidencias (similares) entre las claves especificadas
- `reg compare nombre_clave1 nombre_clave2 /oa` : genera un resultado con las diferencias y coincidencias (ambas) entre las claves especificadas
- `reg compare nombre_clave1 nombre_clave2 /on` : compara dos claves, outputando nada
