# podman ps

Listar contêineres do Podman.

## Usage
- `podman ps` : lista contêineres do Podman em execução atualmente
- `podman ps -a` : lista todos os contêineres do Podman (em execução e parados)
- `podman ps -l` : mostra o contêiner mais recente criado (inclui todos os estados)
- `podman ps -f "name=nome"` : filtra contêineres que contêm uma substring em seus nomes
- `podman ps -f "ancestor=imagem:tag"` : filtra contêineres que compartilham uma determinada imagem como ancestral
- `podman ps -a -f "exited=código"` : filtra contêineres pelo código de status de saída
- `podman ps -f "status=status"` : filtra contêineres pelo status (criado, em execução, removendo, pausado, encerrado e morto)
- `podman ps -f "volume=caminho/para/diretório" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filtra contêineres que montam um volume específico ou têm um volume montado em um caminho específico

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-ps.1.html
