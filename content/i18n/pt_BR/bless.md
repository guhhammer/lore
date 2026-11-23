# bless

Define a capacidade de inicialização por volume e as opções de disco de inicialização. Set volume boot capability and startup disk options.

## Usage
- `bless --folder /Volumes/Mac OS X/System/Library/CoreServices --bootinfo --bootefi` : define um volume somente com Mac OS X ou Darwin e cria os arquivos BootX e `boot.efi` se necessário
- `bless --mount /Volumes/Mac OS --setBoot` : define um volume contendo Mac OS 9 ou Mac OS X como o volume ativo
- `bless --netboot --server bsdp://255.255.255.255` : define o sistema para NetBoot e transmite para um servidor disponível
- `bless --info --plist` : coleta informações sobre o volume atualmente selecionado (conforme determinado pelo firmware), adequado para piping para um programa capaz de analisar listas de propriedades

## Links
- More information: https://keith.github.io/xcode-man-pages/bless.8.html
