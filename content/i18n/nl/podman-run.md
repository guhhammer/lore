# podman run

Voer een commando uit in een nieuwe Podman-container.

## Usage
- `podman run image:tag commando` : voer een commando uit in een nieuwe container op basis van een getagde image
- `podman run -d image:tag commando` : voer een commando uit in een nieuwe container op de achtergrond en toon de ID ervan
- `podman run --rm -it image:tag commando` : voer een commando uit in een eenmalige container in interactieve modus en pseudo-tty
- `podman run -e 'variabele=waarde' -e variabele image:tag commando` : voer een commando uit in een nieuwe container met doorgegeven omgevingsvariabelen
- `podman run -v /pad/naar/hostpad:/pad/naar/containerpad image:tag commando` : voer een commando uit in een nieuwe container met gebonden volumes
- `podman run -p hostpoort:containerpoort image:tag commando` : voer een commando uit in een nieuwe container met gepubliceerde poorten
- `podman run --entrypoint commando image:tag` : voer een commando uit in een nieuwe container waarbij het entrypoint van de image wordt overschreven
- `podman run --network netwerk image:tag` : voer een commando uit in een nieuwe container en verbind deze met een netwerk

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-run.1.html
