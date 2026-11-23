# aws s3

CLI für AWS S3. AWS S3 stellt Speicherplatz in der Cloud zur Verfügung.

## Usage
- `aws s3 ls bucket_name` : liste alle Objekte in einem Bucket auf
- `aws s3 sync pfad/zu/datei_oder_verzeichnis s3://bucket_name` : synchronisiere Dateien und Verzeichnissen zu einem Bucket
- `aws s3 sync s3://bucket_name pfad/zu/ziel` : synchronisiere Dateien und Verzeichnissen von einem Bucket
- `aws s3 sync pfad/zu/datei_oder_verzeichnis s3://bucket_name --exclude pfad/zu/datei --exclude pfad/zu/verzeichnis/*` : synchronisiere Dateien und Verzeichnissen mit Ausnahmen
- `aws s3 rm s3://bucket/pfad/zu/datei` : entferne ein Objekt von einem Bucket
- `aws s3 befehl --dryrun` : probelauf eines angegebenen Kommandos ohne dieses auszuführen

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/
