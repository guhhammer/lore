# astyle

Indentador, formateador y embellecedor de código fuente para los lenguajes de programación C, C++, C# y Java. Al ejecutarse, se crea una copia del archivo original con un `.orig` añadido al nombre del archivo original.

## Usage
- `astyle archivo_de_origen` : aplica el estilo por defecto de 4 espacios por sangría y sin cambios de formato
- `astyle -A2 ruta/al/archivo` : aplica el estilo Java con llaves adjuntas
- `astyle -A1 ruta/al/archivo` : aplica el estilo allman con llaves discontinuas
- `astyle -s2..20 ruta/al/archivo` : aplica una sangría personalizada utilizando espacios. Elige entre 2 y 20 espacios
- `astyle -t2..20 ruta/al/archivo` : aplica una sangría personalizada utilizando tabuladores. Elige entre 2 y 20 tabulaciones

## Links
- More information: https://manned.org/astyle
