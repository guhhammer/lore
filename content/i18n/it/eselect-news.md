# eselect news

Un modulo `eselect` per la lettura degli articoli di notizie di Gentoo. Nota: Portage stamperà un avviso quando un repository viene sincronizzato e ci sono articoli di notizie non letti.

## Usage
- `eselect news list all|new` : elenca gli articoli di notizie disponibili con i loro numeri (tutti di default)
- `eselect news read numero1 numero2 ...` : stampa gli articoli di notizie specificati
- `eselect news read` : stampa tutti gli articoli di notizie non letti
- `eselect news unread numero1 numero2 ...` : segna gli articoli di notizie specificati come non letti
- `eselect news purge` : elimina tutti gli articoli di notizie letti
- `eselect news count all|new` : stampa il numero di articoli di notizie disponibili (nuovi di default)

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#News
