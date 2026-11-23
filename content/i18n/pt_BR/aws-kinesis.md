# aws kinesis

CLI oficial da AWS para o serviço de streamin de dados Amazon Kinesis.

## Usage
- `aws kinesis list-streams` : lista todos os streams de uma conta
- `aws kinesis put-record --stream-name nome --partition-key chave --data mensagem_codificaca_em_base64` : escreve um registro para um stream Kinesis
- `aws kinesis put-record --stream-name nome --partition-key chave --data "$( echo "minha mensagem não codificada" | base64 )"` : escreve um registro para um stream Kinesis com codificação Base64 inline
- `aws kinesis list-shards --stream-name nome` : lista os shards disponíveis em um stream
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name nome --shard-id id` : obtém uma iteração de shards para leitura da mensagem mais antiga no shard do stream
- `aws kinesis get-records --shard-iterator iteração` : lê registros de um shard usando uma iteração de um shard
