# awslogs

CLI um Log-Gruppen, Streams und Events von Amazon CloudWatch Logs abzurufen.

## Usage
- `awslogs groups` : liste alle Log-Gruppen auf
- `awslogs streams /var/log/syslog` : liste alle bestehenden Streams einer angegebenen Loggruppe auf
- `awslogs get /var/log/syslog -s '2h ago' -e '1h ago'` : rufe alle Logs für jegliche Streams in der angegebenen Log-Gruppe für die letzten 1 bis 2 Stunden ab
- `awslogs get /aws/lambda/meine_lambda_gruppe --filter-pattern 'ERROR'` : rufe alle Logs für einen bestimmten CloudWatch-Logs Filter-Ausdruck ab
- `awslogs get /var/log/syslog ALL --watch` : beobachte Logs für jegliche Streams in der angegebenen Log-Gruppe

## Links
- More information: https://github.com/jorgebastida/awslogs#options
