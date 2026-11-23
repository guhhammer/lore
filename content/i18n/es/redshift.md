# redshift

Ajusta la temperatura de color de una pantalla en función de su entorno. Nota: Redshift no es compatible con Wayland.

## Usage
- `redshift -t 5700:3600` : activa Redshift con la temperatura ajustada a 5700k durante el día y a 3600k por la noche
- `redshift -l latitud:longitud` : activa Redshift con una localización personalizada especificada manualmente
- `redshift -b 0.7:0.4` : activa Redshift con el brillo de la pantalla ajustado al 70% durante el día y al 40% por la noche
- `redshift -g red:green:blue` : activa Redshift con niveles de gamma personalizados (entre 0 y 1)
- `redshift -PO temperatura` : purga los cambios de temperatura existentes y establece una temperatura de color constante e inalterable en modo One-shot

## Links
- More information: https://manned.org/redshift
