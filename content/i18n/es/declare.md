# declare

Declara variables y les da atributos.

## Usage
- `declare variable="valor"` : declara una variable de cadena con el valor especificado
- `declare -i variable="valor"` : declara una variable entera con el valor especificado
- `declare -a variable=(artículo_a artículo_b artículo_c)` : declare una variable de matriz con el valor especificado
- `declare -A variable=([key_a]=item_a [key_b]=item_b [key_c]=item_c)` : declare una variable de matriz asociativa con el valor especificado
- `declare -r variable="valor"` : declara una variable de cadena de sólo lectura con el valor especificado
- `declare -g variable="valor"` : declara una variable global dentro de una función con el valor especificado
- `declare -f nombre_función` : imprime la definición de una función

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-declare
