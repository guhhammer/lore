# aws lightsail

Gestisce le risorse Amazon Lightsail.

## Usage
- `aws lightsail get-instances` : elenca tutti i server virtuali privati, o istanze
- `aws lightsail list-bundles` : elenca tutti i bundle (piani istanza)
- `aws lightsail list-blueprints` : elenca tutte le immagini istanza disponibili, o blueprint
- `aws lightsail create-instances --instance-names nome --availability-zone regione --bundle-id nano_2_0 --blueprint-id id_blueprint` : crea un'istanza
- `aws lightsail get-instance-state --instance-name nome` : stampa lo stato di un'istanza specifica
- `aws lightsail stop-instance --instance-name nome` : ferma un'istanza specifica
- `aws lightsail delete-instance --instance-name nome` : elimina un'istanza specifica

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lightsail/
