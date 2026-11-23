# virsh

Gerenciar domínios de convidados do virsh. (NOTA: 'guest_id' pode ser o ID, nome ou UUID do convidado). Alguns subcomandos, como `list`, têm sua própria documentação de uso.

## Usage
- `virsh connect qemu:///system` : conecta a uma sessão do hipervisor
- `virsh list --all` : lista todos os domínios
- `virsh dumpxml guest_id >caminho/para/convidado.xml` : despeja arquivo de configuração do convidado
- `virsh create caminho/para/arquivo_de_configuracao.xml` : cria um convidado a partir de um arquivo de configuração
- `virsh edit guest_id` : edita o arquivo de configuração de um convidado (o editor pode ser alterado com `$EDITOR`)
- `virsh comando guest_id` : inicia/reinicia/desliga/suspende/resume um convidado
- `virsh save guest_id nome_do_arquivo` : salva o estado atual de um convidado em um arquivo
- `virsh destroy guest_id && virsh undefine guest_id` : exclui um convidado em execução

## Links
- More information: https://libvirt.org/manpages/virsh.html
