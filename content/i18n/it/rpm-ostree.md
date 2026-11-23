# rpm-ostree

Un sistema ibrido immagine/pacchetto. Gestisce i deployment ostree, i layer di pacchetti, gli overlay del filesystem e la configurazione di boot.

## Usage
- `rpm-ostree status` : mostra i deployment rpm-ostree nell’ordine in cui appariranno nel bootloader
- `rpm-ostree upgrade --preview` : mostra i pacchetti obsoleti che possono essere aggiornati
- `rpm-ostree upgrade -r` : prepara un nuovo deployment ostree con i pacchetti aggiornati e riavvia in esso
- `rpm-ostree rollback -r` : riavvia nel deployment ostree precedente
- `rpm-ostree install pacchetto -r` : installa un pacchetto in un nuovo deployment ostree e riavvia in esso

## Links
- More information: https://coreos.github.io/rpm-ostree/administrator-handbook/
