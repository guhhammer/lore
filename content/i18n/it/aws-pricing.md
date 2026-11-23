# aws pricing

Interroga servizi, prodotti e informazioni sui prezzi da Amazon Web Services.

## Usage
- `aws pricing describe-services --region us-east-1` : elenca i codici servizio di una regione specifica
- `aws pricing describe-services --service-code AmazonEC2 --region us-east-1` : elenca gli attributi per un dato codice servizio in una regione specifica
- `aws pricing get-products --service-code AmazonEC2 --region us-east-1` : stampa le informazioni sui prezzi per un codice servizio in una regione specifica
- `aws pricing get-attribute-values --service-code AmazonEC2 --attribute-name instanceType --region us-east-1` : elenca i valori per un attributo specifico per un codice servizio in una regione specifica
- `aws pricing get-products --service-code AmazonEC2 --filters "Type=TERM_MATCH,Field=instanceType,Value=m5.xlarge" "Type=TERM_MATCH,Field=location,Value=US East (N. Virginia)" --region us-east-1` : stampa le informazioni sui prezzi per un codice servizio utilizzando filtri per tipo istanza e posizione

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/pricing/
