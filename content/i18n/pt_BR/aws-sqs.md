# aws sqs

Cria, apaga, e envia mensagens para filas para o serviço AWS SQS.

## Usage
- `aws sqs list-queues` : lista todas as filas disponíveis
- `aws sqs get-queue-url --queue-name nome_da_fila` : exibe a URL de uma fila específica
- `aws sqs create-queue --queue-name nome_da_fila --attributes file://caminho/para/arquivos_de_atributos.json` : cria uma fila com atributos especificados em arquivo JSON
- `aws sqs send-message --queue-url https://sqs.regiao.amazonaws.com/nome_da_fila --message-body "corpo_da_mensagem" --delay-seconds inteiro --message-attributes file://caminho/para/arquivos_de_atributos.json` : envia mensagem específica para uma fila
- `aws sqs delete-message --queue-url https://url_da_fila --receipt-handle identificado_da_mensagem` : remove uma mensagem específica de uma fila
- `aws sqs delete-queue --queue-url https://sqs.regiao.amazonaws.com/nome_da_fila` : remove uma fila específica
- `aws sqs purge-queue --queue-url https://sqs.regiao.amazonaws.com/nome_da_fila` : remove todas as mensagens de uma fila específica
- `aws sqs add-permission --queue-url https://sqs.regiao.amazonaws.com/nome_da_fila --label nome_da_permissao --aws-account-ids id_da_conta --actions SendMessage` : habilita uma conta AWS específica para enviar mensagens para uma fila

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sqs/
