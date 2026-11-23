# aws

Das offizielle CLI für Amazon Web Services. Manche Unterbefehle wie `s3` sind separat dokumentiert.

## Usage
- `aws configure wizard` : konfiguriere die AWS Kommandozeile
- `aws configure sso` : konfiguriere die AWS Kommandozeile mithilfe von SSO
- `aws sts get-caller-identity` : zeige Informationen über die eigene angenommene Identität (häufig benutzt zur Fehlersuche)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : liste alle AWS Ressourcen in einer Region mit YAML Formatierung auf
- `aws iam create-user --cli-auto-prompt` : erstelle einen IAM Benutzer mit Ausführungsassistent
- `aws dynamodb wizard neue_tabelle` : öffne einen Assistenten für eine AWS Ressource
- `aws dynamodb update-table --generate-cli-skeleton` : erstelle einen JSON CLI-Aufbau (hilfreich für Infrastruktur-Automation)
- `aws befehl help` : zeige Hilfe für ein Kommando an

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
