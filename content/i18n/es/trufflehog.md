# trufflehog

Encuentra y verifica credenciales en archivos, repositorios de Git, cubos S3 e imágenes Docker.

## Usage
- `trufflehog git https://github.com/trufflesecurity/test_keys --only-verified` : escanea un repositorio de Git en busca de secretos verificados
- `trufflehog github --org trufflesecurity --only-verified` : escanea una organización de GitHub en busca de secretos verificados
- `trufflehog git https://github.com/trufflesecurity/test_keys --only-verified --json` : escanea un repositorio de GitHub en busca de claves verificadas y genera un archivo de salida JSON
- `trufflehog github --repo https://github.com/trufflesecurity/test_keys --issue-comments --pr-comments` : escanea un repositorio de GitHub junto con sus incidencias y solicitudes de extracción
- `trufflehog s3 --bucket nombre_del_cubo --only-verified` : escanea un cubo S3 en busca de claves verificadas
- `trufflehog s3 --role-arn iam-role-arn` : escanea cubos S3 utilizando roles IAM
- `trufflehog filesystem ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : escanea archivos o directorios individuales
- `trufflehog docker --image trufflesecurity/secrets --only-verified` : escanea una imagen de Docker en busca de secretos verificados

## Links
- More information: https://github.com/trufflesecurity/trufflehog#memo-usage
