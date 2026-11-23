# amass enum

Busca subdominios de un dominio.

## Usage
- `amass enum -d nombre_dominio` : encuentra (pasivamente) subdominios de un dominio
- `amass enum -active -d nombre_dominio -p 80,443,8080` : encuentra subdominios de un dominio y los verifica activamente intentando resolver los subdominios encontrados
- `amass enum -brute -d nombre_dominio` : realiza una búsqueda de subdominios por fuerza bruta
- `amass enum -o archivo_de_salida -d nombre_dominio` : guarda los resultados en un archivo de texto
- `amass enum -o fichero_de_salida -dir ruta/al/directorio -d nombre_dominio` : guarda la salida del terminal en un archivo y otros resultados detallados en un directorio
- `amass enum -list` : lista todas las fuentes de datos disponibles
