# aws s3 ls

Elenca bucket AWS S3, cartelle (prefissi) e file (oggetti).

## Usage
- `aws s3 ls` : elenca tutti i bucket
- `aws s3 ls s3://nome_bucket` : elenca file e cartelle nella radice di un bucket (`s3://` è opzionale)
- `aws s3 ls nome_bucket/percorso/della/directory/` : elenca file e cartelle direttamente all'interno di una directory
- `aws s3 ls --recursive nome_bucket` : elenca tutti i file in un bucket
- `aws s3 ls --recursive nome_bucket/percorso/della/directory/prefisso` : elenca tutti i file in un percorso con un prefisso dato
- `aws s3 ls help` : mostra l'aiuto

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/ls.html
