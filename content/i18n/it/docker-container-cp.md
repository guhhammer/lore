# docker container cp

Copia file o directory tra il filesystem di un container e quello locale (host).

## Usage
- `docker cp percorso/del/file_o_directory_su_host nome_container:percorso/del/file_o_directory_su_container` : copia un file o una directory dall'host a un container
- `docker cp nome_container:percorso/del/file_o_directory_su_container percorso/del/file_o_directory_su_host` : copia un file o una directory da un container all'host
- `docker cp -L percorso/del/link_simbolico_su_host nome_container:percorso/del/file_o_directory_su_container` : copia un file o una directory dall'host a un container, seguendo un link simbolico (non copiare il link simbolico, ma direttamente il file da lui referenziato)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
