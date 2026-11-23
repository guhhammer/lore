# argon2

Calcula hashes criptográficos Argon2.

## Usage
- `echo "contraseña" | argon2 "texto_salt"` : calcula un hash con una contraseña y un salt con los parámetros por defecto
- `echo "contraseña" | argon2 "texto_sal" -d|i|id` : calcula un hash con el algoritmo especificado
- `echo "contraseña" | argon2 "texto_sal" -e` : muestra el hash de salida sin información adicional
- `echo "contraseña" | argon2 "texto_sal" -t 5 -m 20 -p 7` : calcula un hash con una cantidad de iteraciones dada, uso de memoria y parámetros de paralelismo dados

## Links
- More information: https://github.com/P-H-C/phc-winner-argon2#command-line-utility
