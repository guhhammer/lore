# aws-vault

Ein Tresor für Entwicklungsumgebungen, um AWS Sicherheitsschlüssel sicher speichern und abrufen zu können.

## Usage
- `aws-vault add profil` : füge einen Sicherheitsschlüssel als Profil zu einem Tresor hinzu
- `aws-vault exec profil -- aws s3 ls` : führe einen Befehl mit AWS Sicherheitsschlüsseln aus dem angegebenen Profil aus
- `aws-vault login profil` : öffne ein Browserfenster für den Login in die AWS Konsole
- `aws-vault list` : liste alle Profile zusammen mit deren Sicherheitsschlüsseln und Sitzungen auf
- `aws-vault rotate profil` : rotiere die AWS Sicherheitsschlüssel für ein Profil
- `aws-vault remove profil` : entferne Sicherheitsschlüsseln eines Profils aus dem Tresor

## Links
- More information: https://github.com/99designs/aws-vault
