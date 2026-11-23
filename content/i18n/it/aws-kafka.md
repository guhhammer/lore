# aws kafka

Gestisce un cluster Amazon MSK (Managed Streaming for Apache Kafka).

## Usage
- `aws kafka create-cluster --cluster-name nome_cluster --broker-node-group-info instanceType=instance_type,clientSubnets=id_subnet1 id_subnet2 ... --kafka-version versione --number-of-broker-nodes numero` : crea un nuovo cluster MSK
- `aws kafka describe-cluster --cluster-arn arn_cluster` : descrivi un cluster MSK
- `aws kafka list-clusters` : elenca tutti i cluster MSK nella regione corrente
- `aws kafka create-configuration --name nome_configurazione --server-properties file://percorso/del/file_di_configurazione.txt` : crea una nuova configurazione MSK
- `aws kafka describe-configuration --arn arm_configurazione` : descrivi una configurazione MSK
- `aws kafka list-configurations` : elenca tutte le configurazioni MSK nella regione corrente
- `aws kafka update-cluster-configuration --cluster-arn arn_cluster --configuration-info arn=arn_configurazione,revision=revisione_configurazione` : aggiorna la configurazione del cluster MSK
- `aws kafka delete-cluster --cluster-arn arn_cluster` : elimina il cluster MSK

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/kafka/
