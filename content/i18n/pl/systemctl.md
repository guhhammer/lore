# systemctl

Kontroluj systemd i menedżera usług.

## Usage
- `systemctl status` : wyświetl wszystkie działające usługi
- `systemctl --failed` : wyświetl nieudane jednostki
- `systemctl start|stop|restart|reload jednostka` : uruchom/Zatrzymaj/Zrestartuj/Przeładuj usługę
- `systemctl status jednostka` : wyświetl status jednostki
- `systemctl enable|disable jednostka` : włącz/Wyłącz automatyczne uruchamianie jednostki przy starcie systemu
- `systemctl mask|unmask jednostka` : zamaskuj/Zdemaskuj jednostkę, aby uniemożliwić włączanie i ręczną aktywację
- `systemctl daemon-reload` : przeładuj systemd, skanując w poszukiwaniu nowych lub zmienionych jednostek
- `systemctl is-enabled jednostka` : sprawdź, czy jednostka jest włączona

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
