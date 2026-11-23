# nixos-rebuild

Rekonfiguriere eine NixOS-Maschine.

## Usage
- `sudo nixos-rebuild switch` : erstelle und wechsle zu einer neuen Konfiguration und nutze diese künftig als Standardkonfiguration
- `sudo nixos-rebuild switch -p name` : gib der neu erstellten Standardkonfiguration einen Namen
- `sudo nixos-rebuild switch --upgrade` : erstelle und wechsle zu einer neuen Konfiguration, nutze diese künftig als Standardkonfiguration und installiere Updates
- `sudo nixos-rebuild switch --rollback` : setze Änderungen der Konfiguration zurück und wechsle zur vorhergehenden Konfiguration
- `sudo nixos-rebuild boot` : erstelle eine neue Konfiguration und starte diese zukünftig direkt ohne sofort zu wechseln
- `sudo nixos-rebuild test` : erstelle und wechsle direkt zu einer neuen Konfiguration, ändere den Standard-Start-Eintrag nicht (dieses Kommando ist für Testzwecke gedacht)
- `sudo nixos-rebuild build-vm` : erstelle die Konfiguration und öffne diese in einer virtuellen Maschine
- `nixos-rebuild list-generations` : liste die verfügbaren Generationen wie im Bootloader-Menü auf

## Links
- More information: https://nixos.org/nixos/manual/#sec-changing-config
