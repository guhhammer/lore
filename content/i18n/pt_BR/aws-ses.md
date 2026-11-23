# aws ses

Interface de linha de comando para o AWS Simple Email Service. Serviço em nuvem com alta performance para envio e recebimento de emails.

## Usage
- `aws ses create-receipt-rule-set --rule-set-name nome_do_conjunto_de_regras --generate-cli-skeleton` : cria um novo conjunto de regras
- `aws ses describe-active-receipt-rule-set --generate-cli-skeleton` : descreve os conjuntos ativos de regras
- `aws ses describe-receipt-rule --rule-set-name nome_do_conjunto_de_regras --rule-name nome_da_regra --generate-cli-skeleton` : descreve um regra específica de um conjunto de regras
- `aws ses list-receipt-rule-sets --starting-token texto_do_token --max-items inteiro --generate-cli-skeleton` : lista todos os conjuntos de regras
- `aws ses delete-receipt-rule-set --rule-set-name nome_do_conjunto_de_regras --generate-cli-skeleton` : remove um conjunto de regras específico (o conjunto ativo não pode ser removido)
- `aws ses delete-receipt-rule --rule-set-name nome_do_conjunto_de_regras --rule-name nome_da_regra --generate-cli-skeleton` : remove uma regras específica de um conjunto de regras
- `aws ses send-email --from de_endereco --destination "ToAddresses=para_endereco" --message "Subject={Data=assunto,Charset=utf8},Body={Text={Data=corpo_email,Charset=utf8},Html={Data=corpo_do_email_com_html,Charset=utf8\}\}"` : envia um email
- `aws ses subcomando help` : exibe ajuda para um subcomando específico do SES

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ses/
