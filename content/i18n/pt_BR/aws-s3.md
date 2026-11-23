# aws s3

Interface de linha de comando para AWS S3. Provê armazenamento através de uma interface de web services.

## Usage
- `aws s3 ls nome_do_bucket` : exibe arquivos de um bucket
- `aws s3 sync caminho/para/arquivos s3://nome_do_bucket` : sincroniza arquivos e diretórios locais para o bucket
- `aws s3 sync s3://nome_do_bucket caminho/para/diretório` : sincroniza arquivos e diretórios do bucket para diretório local
- `aws s3 sync caminho/para/arquivos s3://nome_do_bucket --exclude caminho/para/arquivo --exclude caminho/não/sincronizado/*` : sincroniza arquivos e diretórios excluindo algo
- `aws s3 rm s3://nome_do_bucket/caminho/para/arquivo` : remove arquivo do bucket
- `aws s3 qualquer_comando --dryrun` : somente exibe a prévia das mudanças

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/
