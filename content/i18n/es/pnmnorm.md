# pnmnorm

Normaliza el contraste en una imagen PNM.

## Usage
- `pnmnorm ruta/a/la/imagen.pnm >ruta/al/resultado.pnm` : fuerza los píxeles más brillantes a ser blancos, los más oscuros hacia negro y disemina los demás linealmente
- `pnmnorm -midv n ruta/a/la/imagen.pnm >ruta/al/resultado.pnm` : fuerza los píxeles más brillantes a ser blancos, los más oscuros hacia negro y disemina los demás cuadráticamente, de tal forma que los píxeles con un brillo de 'n' tienen un 50 % del brillo
- `pnmnorm -k ruta/a/la/imagen.pnm >ruta/al/resultado.pnm` : mantiene el tono (hue) de los píxeles, solo modifica el brillo
- `pnmnorm -luminosity|colorvalue|saturation ruta/a/la/imagen.pnm >ruta/al/resultado.pnm` : especifica un método para calcular el brillo de un píxel

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmnorm.html
