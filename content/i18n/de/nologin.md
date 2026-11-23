# nologin

Alternative Shell, die verhindert, dass sich ein Benutzer einloggt.

## Usage
- `chsh -s user nologin` : setze die Login-Shell eines Benutzers auf `nologin`, um zu verhindern, dass der Benutzer sich anmeldet
- `echo "nachricht" >/etc/nologin.txt` : passe die Nachricht für Benutzer mit Login-Shell `nologin` an

## Links
- More information: https://manned.org/nologin.8
