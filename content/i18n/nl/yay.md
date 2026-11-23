# yay

Yet Another Yogurt: bouw en installeer pakketten van de Arch User Repository.

## Usage
- `yay pakket_naam|zoekpatroon` : zoek en installeer interactief pakketten van de bronnen en AUR
- `yay` : synchroniseer en update alle pakketten van de bronnen en AUR
- `yay -S pakket --noconfirm` : installeer een nieuw pakket van de bronnen en AUR en vraag niet om transacties te bevestigen
- `yay -Rns pakket` : verwijder een geïnstalleerd pakket en de bijbehorende afhankelijkheden en configuratiebestanden
- `yay -Ss sleutelwoord` : doorzoek de pakketdatabase voor een sleutelwoord van de bronnen en AUR
- `yay -Yc` : verwijder weespakketten (geïnstalleerd als afhankelijkheden maar niet daadwerkelijk vereist door een ander pakket)
- `yay -Scc` : leeg de `pacman` en `yay` caches (oude pakketversies worden bewaard voor rollback- en downgrade-doeleinden)
- `yay -Ps` : toon statistieken van geïnstalleerde pakketten en systeemstatus

## Links
- More information: https://github.com/Jguer/yay#first-use
