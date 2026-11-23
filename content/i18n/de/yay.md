# yay

Yet Another Yogurt: Ein Programm für Arch Linux um Pakete vom Arch User Repository zu installieren.

## Usage
- `yay paketname|suchbegriff` : suche und installiere Pakete von den Repositorys und dem AUR interaktiv
- `yay` : synchronisiere und aktualisiere alle Pakete von den Repositorys und dem AUR
- `yay -Sua` : synchronisiere und aktualisiere nur AUR-Pakete
- `yay -S paketname` : installiere ein neues Paket von den Repositorys und dem AUR
- `yay -Rns paketname` : entferne ein Paket sowie alle Abhängigkeiten und Konfigurationsdateien
- `yay -Ss schlüsselwort` : suche in der Paketdatenbank nach einem Schlüsselwort in den Repositorys und dem AUR
- `yay -Yc` : entferne verwaiste Pakete (als Abhängigkeit installiert, aber von keinem Paket benötigt)
- `yay -Ps` : zeige Statistiken für installierte Pakete sowie die Gesundheit des Systems an

## Links
- More information: https://github.com/Jguer/yay#first-use
