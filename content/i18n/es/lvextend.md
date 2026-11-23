# lvextend

Aumenta el tamaño de un volumen lógico.

## Usage
- `sudo lvextend -L 120G volumen_logico` : aumenta el tamaño de un volumen a 120 GB
- `sudo lvextend -L +40G -r volumen_logico` : aumenta el tamaño de un volumen por 40 GB, así como a los sistemas subyacentes
- `sudo lvextend -l +100%FREE volumen_logico` : aumenta el tamaño de un volumen al 100% del espacio físico libre del volumen
- `sudo lvextend -l +100%FREE -r volumen_logico` : aumenta el tamaño de un volumen al 100% del espacio físico libre del volumen y redimensiona a los sistemas subyacentes

## Links
- More information: https://manned.org/lvextend
