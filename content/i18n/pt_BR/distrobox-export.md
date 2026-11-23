# distrobox-export

Exportar um aplicativo/serviço/binário do contêiner para o sistema operacional host. Subcomando de `distrobox`.

## Usage
- `distrobox-export -a nome_do_pacote -ef "--foreground"` : exporta um aplicativo do contêiner para o host (a entrada e o ícone do aplicativo aparecerão na lista de aplicativos do seu sistema host)
- `distrobox-export -b caminho/para/binário -ep caminho/para/binário_no_host` : exporta um binário do contêiner para o host
- `distrobox-export -b caminho/para/binário -ep caminho/de/exportação` : exporta um binário do contêiner para o host (por exemplo, `$HOME/.local/bin`)
- `distrobox-export --service pacote -ef "--allow-newer-config" -S` : exporta um serviço do contêiner para o host (`--sudo` executará o serviço como root dentro do contêiner)
- `distrobox-export -a pacote -d` : desexportar/deletar um aplicativo exportado

## Links
- More information: https://distrobox.it/usage/distrobox-export/
