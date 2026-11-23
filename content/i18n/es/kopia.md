# kopia

Herramienta de copia de seguridad de código abierto, rápida y segura. Soporta encriptación, compresión, deduplicación e instantáneas incrementales.

## Usage
- `kopia repository create filesystem --path ruta/al/repositorio_local` : crea un repositorio en el sistema de archivos local
- `kopia repository create s3 --bucket nombre_del_bucket --access-key identificador_de_clave_de_acceso_AWS --secret-access-key clave_de_acceso_secreta_AWS` : crea un repositorio en Amazon S3
- `kopia repository connect tipo_de_repositorio --path ruta/al/repositorio` : conecta a un repositorio
- `kopia snapshot create ruta/al/directorio` : crea una instantánea de un directorio
- `kopia snapshot list` : lista instantáneas
- `kopia snapshot restore identificador_de_instantánea ruta/al/directorio_objetivo` : restaura una instantánea en un directorio específico
- `kopia policy set --global --keep-latest número_de_instantáneas_a_mantener --compression algoritmo_de_compresión` : crea una nueva política
- `kopia policy set --global --add-ignore ruta/al/archivo_o_directorio` : excluye un archivo o directorio específico de las copias de seguridad

## Links
- More information: https://kopia.io/docs/reference/command-line/
