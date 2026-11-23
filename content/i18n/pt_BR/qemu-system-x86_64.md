# qemu-system-x86_64

Emula a arquitetura `x86_64`.

## Usage
- `qemu-system-x86_64 -hda nome_da_imagem.img -m 4096` : inicializa a partir de uma imagem emulando a arquitetura `x86_64`
- `qemu-system-x86_64 -hda nome_da_imagem.img -m 4096 -cdrom imagem_do_sistema.iso -boot d` : inicializa uma instância do QEMU a partir de uma imagem live ISO
- `qemu-system-x86_64 -hda /dev/dispositivo_de_armazenamento -m 4096` : inicializa a partir de um dispositivo físico (ex.: a partir do USB para testar uma mídia inicializável)
- `qemu-system-x86_64 -hda nome_da_imagem.img -m 4096 -nographic` : não inicia um servidor VNC
- `<Ctrl a><x>` : sai do QEMU sem gráficos
- `qemu-system-x86_64 -M help` : lista os tipos de máquina suportados

## Links
- More information: https://www.qemu.org/docs/master/system/target-i386.html
