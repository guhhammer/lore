# pamcomp

Superpone dos imágenes PAM.

## Usage
- `pamcomp ruta/a/overlay.pam ruta/a/underlay.pam >ruta/a/output.pam` : superpone dos imágenes de tal forma que la superposición bloquee partes de la imagen subyacente
- `pamcomp -ali left|center|right|beyondleft|beyondright -x x_offset ruta/a/overlay.pam ruta/a/underlay.pam >ruta/a/output.pam` : establece la alineación horizontal de la superposición
- `pamcomp -va top|middle|bottom|above|below -y y_offset ruta/a/overlay.pam ruta/a/underlay.pam >ruta/a/output.pam` : establece la alineación vertical de la superposición
- `pamcomp -o 0.7 ruta/a/overlay.pam ruta/a/underlay.pam >ruta/a/output.pam` : establece la opacidad de la superposición

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcomp.html
