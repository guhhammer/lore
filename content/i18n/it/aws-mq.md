# aws mq

Definisce e gestisce broker di messaggi in AWS.

## Usage
- `aws mq create-broker --host-instance-type tipo_istanza --broker-name nome_broker --engine-type ACTIVEMQ|RABBITMQ --publicly-accessible|--no-publicly-accessible` : crea un broker
- `aws mq list-brokers` : elenca tutti i broker
- `aws mq describe-broker --broker-id id_broker` : mostra i dettagli di un broker specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/mq/
