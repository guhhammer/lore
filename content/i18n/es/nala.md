# nala

Gestor de paquetes con interfaz moderna. Interfaz para la API `python-apt`.

## Usage
- `sudo nala install paquete` : instala un paquete o lo actualiza a la última versión disponible
- `sudo nala remove paquete` : desinstala un paquete
- `nala purge paquete` : desinstala un paquete y borra sus archivos de configuración
- `nala search "patrón"` : busca nombres de paquetes y sus descripciones mediante una palabra, una `regex` (por defecto) o mediante `glob`
- `sudo nala upgrade` : actualiza la lista de paquetes disponibles e instala las actualizaciones
- `sudo nala autoremove` : elimina todos los paquetes y dependencias no utilizados de tu sistema
- `sudo nala fetch` : búsqueda de los espejos más rápidos para mejorar tiempos de descarga
- `nala history` : visualiza el histórico de transacciones

## Links
- More information: https://gitlab.com/volian/nala
