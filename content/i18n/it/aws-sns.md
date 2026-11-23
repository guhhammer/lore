# aws sns

Crea topic e sottoscrizioni, invia e riceve messaggi, e monitora eventi e log per Amazon Simple Notification Service.

## Usage
- `aws sns list-origination-numbers|phone-numbers-opted-out|platform-applications|sms-sandbox-phone-numbers|subscriptions|topics` : elenca tutti gli oggetti di un tipo specifico
- `aws sns create-topic --name nome` : crea un topic con un nome specifico e mostra il suo Amazon Resource Name (ARN)
- `aws sns subscribe --topic-arn arn_topic --protocol email --notification-endpoint email` : sottoscrive un indirizzo email al topic con un ARN specifico e mostra l'ARN della sottoscrizione
- `aws sns publish --topic-arn "arn:aws:sns:us-west-2:123456789012:topic-name"|--phone-number +1-555-555-0100 --message file://percorso/del/file` : pubblica un messaggio a un topic o numero di telefono specifico e mostra l'ID del messaggio
- `aws sns unsubscribe --subscription-arn ARN_sottoscrizione` : elimina la sottoscrizione con un ARN specifico dal suo topic
- `aws sns create-platform-endpoint --platform-application-arn platform_application_ARN --token token` : crea un endpoint piattaforma
- `aws sns add-permission --topic-arn arn_topic --label topic_label --aws-account-id account_id --action-name AddPermission|CreatePlatformApplication|DeleteEndpoint|GetDataProtectionPolicy|GetEndpointAttributes|Subscribe|...` : aggiunge un'istruzione alla policy di controllo accessi di un topic
- `aws sns tag-resource --resource-arn arn_topic --tags Key=tag1_key Key=tag2_key,Value=tag2_value ...` : aggiunge un tag al topic con un ARN specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sns/
