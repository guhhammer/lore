# docker container run

Voer een commando uit in een nieuwe Docker-container.

## Usage
- `docker run image:tag commando` : voer een commando uit in een nieuwe container vanuit een getagd image
- `docker run -d image commando` : voer een commando uit in een nieuwe container op de achtergrond en toon de ID ervan
- `docker run --rm -it image commando` : voer een commando uit in een eenmalige container in interactieve modus en pseudo-TTY
- `docker run -e 'variabele=waarde' -e variabele image commando` : voer een commando uit in een nieuwe container met doorgegeven omgevingsvariabelen
- `docker run -v /pad/naar/host_pad:/pad/naar/container_pad image commando` : voer een commando uit in een nieuwe container met gebonden volumes
- `docker run -p host_poort:container_poort image commando` : voer een commando uit in een nieuwe container met gepubliceerde poorten
- `docker run --entrypoint commando image` : voer een commando uit in een nieuwe container en overschrijf het entrypoint van de image
- `docker run --network netwerk image` : voer een commando uit in een nieuwe container die deze verbindt met een netwerk

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
