# pactrans

Installeer, verwijder en upgrade ALPM-pakketten.

## Usage
- `sudo pactrans --install pakket_naam` : installeer een pakket vanuit een repository
- `sudo pactrans --remove pakket_naam` : verwijder een pakket
- `sudo pactrans --sysupgrade` : upgrade alle geïnstalleerde pakketten
- `sudo pactrans --file pad/naar/pakket.pkg.tar.zst` : installeer een pakketbestand
- `sudo pactrans local/pakket_om_te_verwijderen repository_naam/pakket_om_te_installeren` : vervang een lokaal geïnstalleerd pakket door een pakket uit een repository
- `pactrans --print-only --install pakket_naam` : toon wat de transactie zou doen zonder deze uit te voeren

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pactrans.pod
