# aws s3 cp

Copia archivos locales u objetos de S3 a otra ubicación, ya sea local o en S3.

## Usage
- `aws s3 cp ruta/al/archivo s3://nombre_bucket/ruta/al/archivo_remoto` : copia un archivo local a un bucket específico
- `aws s3 cp s3://nombre_bucket1/ruta/al/archivo s3://nombre_bucket2/ruta/de/destino` : copia un objeto específico de S3 a otro bucket
- `aws s3 cp s3://nombre_bucket1/ruta/al/archivo s3://nombre_bucket2` : copia un objeto específico de S3 a otro bucket manteniendo el nombre original
- `aws s3 cp s3://nombre_bucket . --recursive` : copia objetos de S3 a un directorio local de forma recursiva
- `aws s3 cp help` : muestra la ayuda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/cp.html
