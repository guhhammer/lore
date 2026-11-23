# choco pin

Przypięcie obecnej bądź podanej wersji dla danego pakietu zarządzanego przez Chocolatey. Przypięte pakiety są automatycznie pomijane podczas aktualizacji pakietów.

## Usage
- `choco pin list` : wyświetlanie listy obecnie przypiętych pakietów wraz z wersjami
- `choco pin add -n pakiet` : przypnij pakiet w jego obecnej wersji
- `choco pin add -n pakiet --version wersja` : przypnij pakiet w podanej wersji
- `choco pin remove -n pakiet` : odepnij dany pakiet

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/pin/
