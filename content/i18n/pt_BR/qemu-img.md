# qemu-img

Cria e manipula imagens de disco virtuais do Quick Emulator.

## Usage
- `qemu-img create caminho/para/arquivo_de_imagem.img gigabytesG` : cria uma imagem de disco com um tamanho específico (em gigabytes)
- `qemu-img info caminho/para/arquivo_de_imagem.img` : mostra informações sobre uma imagem de disco
- `qemu-img resize caminho/para/arquivo_de_imagem.img gigabytesG` : aumenta ou diminui o tamanho da imagem
- `qemu-img map caminho/para/arquivo_de_imagem.img` : exibe o estado de alocação de cada setor da imagem de disco especificada
- `qemu-img convert -f vmdk -O qcow2 -p caminho/para/arquivo_de_imagem.vmdk caminho/para/arquivo_de_imagem.qcow2` : converte uma imagem de disco VMware `.vmdk` para uma imagem de disco `.qcow2` do KVM enquanto mostra o progresso
- `qemu-img snapshot -c nome_da_snapshot caminho/para/arquivo_de_imagem.qcow2` : cria uma snapshot interna de uma imagem de disco `.qcow2` do KVM
- `qemu-img snapshot -a nome_da_snapshot caminho/para/arquivo_de_imagem.qcow2` : aplica uma snapshot interna em uma imagem de disco `.qcow2` do KVM

## Links
- More information: https://qemu.readthedocs.io/en/master/tools/qemu-img.html
