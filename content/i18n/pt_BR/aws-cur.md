# aws cur

Cria, pesquisa e apaga relatórios de uso do AWS.

## Usage
- `aws cur put-report-definition --report-definition file://caminho/para/definição_do_relatório.json` : cria um relatório de uso e custo do AWS definido de a partir de um arquivo JSON
- `aws cur describe-report-definitions` : lista as definições dos relatórios de uso para a conta logada
- `aws cur --region região_aws delete-report-definition --report-name relatório` : apaga uma definição de relatório de uso

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
