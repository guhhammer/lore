# debsecan

Analizador de seguridad de Debian, es una herramienta para enumerar vulnerabilidades en una instalación Debian en particular.

## Usage
- `debsecan` : lista de paquetes instalados vulnerables en el host actual
- `debsecan --suite nombre_de_la_versión` : lista de paquetes instalados vulnerables de una versión específica
- `debsecan --suite nombre_de_la_versión --only-fixed` : lista solo vulnerabilidades arregladas
- `debsecan --suite sid --only-fixed --format report --mailto root --update-history` : lista solo vulnerabilidades arregladas de inestable ("sid") y envía un correo a root
- `sudo apt upgrade $(debsecan --only-fixed --format paquetes)` : actualiza paquetes vulnerables instalados

## Links
- More information: https://manned.org/debsecan
