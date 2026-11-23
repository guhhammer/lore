# awslogs

Interroga gruppi, stream ed eventi dai log Amazon CloudWatch.

## Usage
- `awslogs groups` : elenca i gruppi di log
- `awslogs streams /var/log/syslog` : elenca gli stream esistenti per il gruppo specificato
- `awslogs get /var/log/syslog -s '2h ago' -e '1h ago'` : ottieni log per qualsiasi stream nel gruppo specificato tra 1 e 2 ore fa
- `awslogs get /aws/lambda/my_lambda_group --filter-pattern 'ERROR'` : ottieni log che corrispondono a un pattern di filtro CloudWatch Logs specifico
- `awslogs get /var/log/syslog ALL --watch` : osserva i log per qualsiasi stream nel gruppo specificato

## Links
- More information: https://github.com/jorgebastida/awslogs#options
