# amass intel

Recopila información de código abierto sobre una organización, como dominios raíz y ASNs.

## Usage
- `amass intel -addr 192.168.0.1-254` : encuentra dominios raíz en un rango de direcciones IP específico
- `amass intel -active -addr 192.168.0.1-254` : usa métodos activos de reconocimiento
- `amass intel -whois -d nombre_de_dominio` : encuentra dominios raíz relacionados con un dominio específico
- `amass intel -org nombre_de_organización` : encuentra ASNs pertenecientes a una [org]anización específica
- `amass intel -asn cadena` : encuentra dominios raíz pertenecientes a un Número de Sistema Autónomo específico
- `amass intel -o ruta/al/archivo_de_salida -whois -d nombre_de_dominio` : guarda los resultados en un archivo de texto
- `amass intel -list` : lista todas las fuentes de datos disponibles
