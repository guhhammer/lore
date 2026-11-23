# aws configure

Gerencia as configurações para o AWS CLI.

## Usage
- `aws configure` : configura interativamente o AWS CLI (cria uma nova configuração ou atualiza a configuração default)
- `aws configure --profile nome_do_profile` : configura interativamente um profile para o AWS CLI (cria um novo profile ou atualiza um que já existae)
- `aws configure get nome` : exibe o valor de uma variável específica de configuração
- `aws configure get nome --profile nome_do_profile` : exibe o valor de uma variável específica de configuração de um profile específico
- `aws configure set nome valor` : altera o valor de uma variável específica de configuração
- `aws configure set nome valor --profile nome_do_profile` : altera o valor de uma variável específica de configuração de um profile específico
- `aws configure list` : lista os entradas da configuração
- `aws configure list --profile profile_name` : lista os entradas da configuração de um profile específico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/configure/
