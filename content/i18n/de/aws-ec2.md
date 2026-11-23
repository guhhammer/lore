# aws ec2

CLI für AWS EC2. AWS EC2 stellt eine sichere und skalierbare Einheit in der AWS Cloud zur Verfügung, um ein schnelleres Entwickeln und Ausrollen von Software zu ermöglichen.

## Usage
- `aws ec2 describe-instances --instance-ids instanz_id` : liste Informationen zu einer bestimmten Instanz auf
- `aws ec2 describe-instances` : liste Informationen zu allen Instanzen auf
- `aws ec2 describe-volumes` : liste Informationen zu allen EC2 Volumen auf
- `aws ec2 delete-volume --volume-id volumen_id` : lösche ein EC2 Volumen
- `aws ec2 create-snapshot --volume-id volumen_id` : erstelle einen Snapshot basierend auf einem EC2 Volumen
- `aws ec2 describe-images` : liste alle verfügbaren AMIs (Amazon Machine Images) auf
- `aws ec2 help` : liste alle verfügbaren EC2 Befehle auf
- `aws ec2 unterbefehl help` : zeige Hilfe für bestimmte EC2 Unterbefehle an

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
