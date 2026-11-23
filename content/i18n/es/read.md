# read

Comando integrado del shell para recuperar datos de `stdin`.

## Usage
- `read variable1 variable2 ...` : almacena los datos que introduzcas desde el teclado en una o varias variables
- `read -a arreglo` : almacena cada una de las siguientes líneas que ingreses como valores de un array
- `read -n recuento_de_caracteres variable` : especifica el número máximo de caracteres que se van a leer
- `read <<<"El apellido es Bond" _ variable1 _ variable2` : asigna varios valores a varias variables
- `read -r variable` : evita que la barra invertida (`\`) actúe como carácter de escape
- `read -p "Ingrese aquí tu entrada: " variable` : muestra un mensaje de solicitud antes de la entrada
- `read -s variable` : no muestra los caracteres tecleados (modo silencioso)
- `comando | while IFS= read -r line; do echo|ls|rm|... "$line"; done` : realiza una acción en cada línea de la salida de un comando

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-read
