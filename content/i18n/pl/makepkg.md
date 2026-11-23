# makepkg

Utwórz pakiet do użycia przez `pacman`-a. Domyślnie używa pliku `PKGBUILD` w aktualnym katalogu roboczym.

## Usage
- `makepkg` : utwórz pakiet
- `makepkg -s` : utwórz pakiet i zainstaluj jego zależności
- `makepkg -s -i` : utwórz pakiet, zainstaluj jego zależności, a następnie zainstaluj utworzony pakiet
- `makepkg --skipchecksums` : utwórz pakiet, ale pomiń sprawdzanie sum kontrolnych źrodeł
- `makepkg -c` : wyczyść katalogi robocze po udanym budowaniu
- `makepkg --verifysource` : zwerifikuj sumy kontrolne źródeł
- `makepkg --printsrcinfo >.SRCINFO` : wygeneruj i zapisz informacje o źródłach do pliku `.SRCINFO`

## Links
- More information: https://manned.org/makepkg
