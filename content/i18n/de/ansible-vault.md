# ansible-vault

Verschlüsselt und entschlüsselt Werte, Datenstrukturen und Dateien innerhalb von Ansible-Projekten.

## Usage
- `ansible-vault create vault_datei` : erstelle eine neue verschlüsselte Vault-Datei mit einer Eingabeaufforderung für ein Passwort
- `ansible-vault create --vault-password-file schlüsseldatei vault_datei` : erstelle eine neue verschlüsselte Vault-Datei mit einer Vault-Schlüsseldatei, um sie zu verschlüsseln
- `ansible-vault encrypt --vault-password-file schlüsseldatei vault_file` : verschlüssle eine vorhandene Datei mit einer optionalen Schlüsseldatei
- `ansible-vault encrypt_string` : verschlüssle eine Zeichenfolge mit dem verschlüsselten Zeichenfolgenformat von Ansible, wobei interaktive Eingabeaufforderungen angezeigt werden
- `ansible-vault view --vault-password-file schlüsseldatei vault_datei` : zeige eine verschlüsselte Datei an, wobei eine Kennwortdatei zum Entschlüsseln verwendet wird
- `ansible-vault rekey --vault-password-file alte_schlüsseldatei --new-vault-password-file neue_schlüsseldatei vault_datei` : verschlüssle eine bereits verschlüsselte Vault Datei mit einer neuen Kennwortdatei neu

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/vault_guide/index.html
