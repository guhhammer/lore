# aws s3 cp

Copia file locali o oggetti S3 in un'altra posizione localmente o in S3.

## Usage
- `aws s3 cp percorso/del/file s3://nome_bucket/percorso/del/remote_file` : copia un file da locale a un bucket specifico
- `aws s3 cp s3://nome_bucket1/percorso/del/file s3://nome_bucket2/percorso/del/target` : copia un oggetto S3 specifico in un altro bucket
- `aws s3 cp s3://nome_bucket1/percorso/del/file s3://nome_bucket2` : copia un oggetto S3 specifico in un altro bucket mantenendo il nome originale
- `aws s3 cp s3://nome_bucket . --recursive` : copia oggetti S3 in una directory locale ricorsivamente
- `aws s3 cp help` : mostra l'aiuto

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/cp.html
