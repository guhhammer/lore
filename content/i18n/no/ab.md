# ab

Apache HTTP server referansemåling verktøy.

## Usage
- `ab -n 100 url` : utfør 100 HTTP GET-forespørsler til en gitt URL
- `ab -n 100 -c 10 url` : utfør 100 HTTP GET-forespørsler, i samtidige grupper på 10, til en URL
- `ab -n 100 -T application/json -p vei/til/fil.json url` : utfør 100 HTTP POST-forespørsler til en URL, med å bruke en JSON-nyttelast fra en fil
- `ab -k url` : bruk HTTP Keep Alive, dvs. utfør flere forespørsler i én HTTP-økt
- `ab -t 60 url` : angi maksimalt antall sekunder å bruke på referansemåling

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
