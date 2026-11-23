# kcat

Apache Kafka producer- en consumenttool.

## Usage
- `kcat -C -t onderwerp -b brokers` : consumeer berichten startend met de nieuwste offset
- `kcat -C -t onderwerp -b brokers -o beginning -e` : consumeer berichten startend met de oudste offset en sluit af nadat het laatste bericht is ontvangen
- `kcat -G groep_id onderwerp -b brokers` : consumeer berichten als een Kafka-consumentgroep
- `echo bericht | kcat -P -t onderwerp -b brokers` : publiceer bericht via het lezen van de `stdin`
- `kcat -P -t onderwerp -b brokers pad/naar/bestand` : publiceer berichten via het lezen van een bestand
- `kcat -L -b brokers` : toon de metadata voor alle onderwerpen en brokers
- `kcat -L -t onderwerp -b brokers` : toon de metadata voor een specifiek onderwerp
- `kcat -Q -t onderwerp:partitie:unix_timestamp -b brokers` : verkrijg de offset voor een onderwerp/partitie voor een specifiek punt in de tijd

## Links
- More information: https://manned.org/kcat
