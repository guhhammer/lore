# hf

Interactúa con Hugging Face Hub. Inicia sesión, gestiona la caché local, carga o descarga archivos.

## Usage
- `hf auth login` : inicia sesión en Hugging Face Hub
- `hf auth whoami` : muestra el nombre del usuario conectado
- `hf auth logout` : cierra sesión
- `hf env` : genera información sobre el entorno
- `hf download --repo-type tipo_repo id_repo nombre_archivo1 nombre_archivo2 ...` : descarga archivos de un repositorio e imprime la ruta (omite los nombres de archivo para descargar todo el repositorio)
- `hf upload --repo-type tipo_repo id_repo ruta/al/archivo_de_repositorio_o_directorio_de_repositorio ruta/al/archivo_de repositorio_o_directorio` : sube una carpeta entera o un archivo a Hugging Face
- `hf cache ls` : escanea la caché para ver los repositorios descargados y su uso del disco

## Links
- More information: https://huggingface.co/docs/huggingface_hub/guides/cli
