# age

Uma simples, moderna e segura ferramenta de criptografia de arquivos.

## Usage
- `age -p -o caminho/para/arquivo_criptografado caminho/para/arquivo_descriptografado` : gera um arquivo criptografado que pode ser descriptografado com uma frase-chave
- `age -r chave_pública -o caminho/para/arquivo_criptografado caminho/para/arquivo_descriptografado` : criptografa arquivo com uma ou mais chaves públicas que são inseridas como literais (repita o argumento `--recipient` para especificar múltiplas chaves públicas)
- `age -R caminho/para/arquivo_de_destinatários -o caminho/para/arquivo_criptografado caminho/para/arquivo_descriptografado` : criptografa arquivo com um ou mais destinatários que são especificadas em um arquivo (um por linha)
- `age -d -o caminho/para/arquivo_descriptografado caminho/para/arquivo_criptografado` : descriptografa um arquivo com uma frase-chave
- `age -d -i caminho/para/arquivo_de_chave_privada -o caminho/para/arquivo_descriptografado caminho/para/arquivo_criptografado` : descriptografa um arquivo com um arquivo de chave privada

## Links
- More information: https://github.com/FiloSottile/age#usage
