# bootctl

Kontroluj ustawienia oprogramowania układowego EFI i zarządzaj programem rozruchowym.

## Usage
- `bootctl status` : wyświetl informacje o oprogramowaniu układowym i programach rozruchowych
- `bootctl list` : wyświetl wszystkie dostępne wpisy programu rozruchowego
- `sudo bootctl reboot-to-firmware true` : ustaw opcję, aby uruchomić oprogramowanie układowe przy następnym rozruchu (podobne do `sudo systemctl reboot --firmware-setup`)
- `bootctl --esp-path=/ścieżka/do/partycji_systemowej_efi/` : podaj ścieżkę do partycji systemowej EFI (domyślnie `/efi/`, `/boot/` lub `/boot/efi`)
- `sudo bootctl install` : zainstaluj `systemd-boot` do partycji systemowej EFI
- `sudo bootctl remove` : usuń wszystkie zainstalowane wersje `systemd-boot` z partycji systemowej EFI

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/bootctl.html
