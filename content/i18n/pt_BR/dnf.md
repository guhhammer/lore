# dnf

Gerenciador de pacotes das distribuições baseadas em RHEL (substituto do yum).

## Usage
- `sudo dnf upgrade` : atualiza os pacotes instalados para suas versões mais atuais
- `dnf search palavra_chave1 palavra_chave2 ...` : busca pacotes com palavras-chave
- `dnf info pacote` : mostra detalhes sobre um determinado pacote
- `sudo dnf install pacote1 pacote2 ...` : instala um novo pacote (use `-y` para responder sim à todos os prompts)
- `sudo dnf remove pacote1 pacote2 ...` : remove um pacote
- `dnf list --installed` : lista pacotes intalados
- `dnf provides comando` : busca por pacotes que fornecem um dado comando
- `dnf history` : mostra todas as operações passadas

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
