# aws s3

CLI per AWS S3 - fornisce spazio di archiviazione tramite le interfacce di Amazon Web Services.

## Usage
- `aws s3 ls nome_bucket` : mostra file in un bucket
- `aws s3 sync percorso/dei/file s3://nome_bucket` : sincronizza file e directory locali su un bucket
- `aws s3 sync s3://nome_bucket percorso/del/file_o_cartella` : sincronizza file e directory da un bucket in locle
- `aws s3 sync percorso/dei/file s3://nome_bucket --exclude percorso/del/file --exclude directory/*` : sincronizza file e directory escludendo alcuni file o directory
- `aws s3 rm s3://bucket/percorso/del/file` : rimuovi un file dal bucket
- `aws s3 qualsiasi_comando --dryrun` : mostra solo un'anteprima dei cambiamenti

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/
