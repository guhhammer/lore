# ab

Apache HTTP server Benchmarking Tool.

## Usage
- `ab -n 100 url` : sende 100 HTTP GET Anfragen an eine URL
- `ab -n 100 -c 10 url` : sende 100 HTTP GET Anfragen an eine URL, wovon bis zu 10 gleichzeitig bearbeitet werden
- `ab -n 100 -T application/json -p pfad/zu/datei.json url` : sende 100 HTTP POST Anfragen mit der in der angegebenen Datei gespeicherten Payload an eine URL
- `ab -k url` : wach halten
- `ab -t 60 url` : lege die maximale Anzahl an Sekunden fest, die das Benchmarking dauern darf

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
