# bosh

Strumento da linea di comando per distribuire e gestire director BOSH.

## Usage
- `bosh alias-env nome_ambiente -e indirizzo_ip|url --ca-cert certificato_ca` : crea un alias locale per un director
- `bosh environments` : elenca ambienti
- `bosh login -e ambiente` : esegui il login al director
- `bosh -e ambiente deployments` : elenca deployment (distribuzioni)
- `bosh -e ambiente vms -d deployment` : elenca le macchine virtuali in un ambiente
- `bosh -e ambiente ssh macchina_virtuale -d deployment` : avvia una sessione SSH in una macchina virtuale
- `bosh -e ambiente upload-stemcell file_stemcell|url` : carica una stemcell
- `bosh -e ambiente cloud-config` : mostra la configurazione cloud attuale

## Links
- More information: https://bosh.io/docs/cli-v2/
