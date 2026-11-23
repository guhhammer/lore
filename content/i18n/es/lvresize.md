# lvresize

Cambia el tamaño de un volumen lógico.

## Usage
- `sudo lvresize -L 120G grupo_de_volumen/volumen_logico` : cambia el tamaño de un volumen lógico a 120 GB
- `sudo lvresize -L +120G -r grupo_de_volumen/volumen_logico` : extiende el tamaño de un volumen lógico así como el de los sistemas subyacentes en 120 GB
- `sudo lvresize -l 100%FREE grupo_de_volumen/volumen_logico` : extiende el tamaño de un volumen lógico al 100% del espacio físico libre del volumen
- `sudo lvresize -L -120G -r grupo_de_volumen/volumen_logico` : reduce el tamaño de un volumen lógico así como a los sistemas subyacentes en 120 GB

## Links
- More information: https://manned.org/lvresize
