# abroot

Utilitário que fornece total imutabilidade e atomicidade ao transacionar entre 2 estados da partição raíz (A<=>B). Atualizações são realizadas usando imagens OCI, para garantir que o sistema sempre estará em um estado consistente.

## Usage
- `sudo abroot pkg add pacote` : adiciona pacotes à imagem local (Nota: após executar esse comando você precisa aplicar as alterações.)
- `sudo abroot pkg remove pacote` : remove pacotes da imagem local(Nota: após executar esse comando você precisa aplicar as alterações.)
- `sudo abroot pkg list` : lista pacotes da imagem local
- `sudo abroot pkg apply` : aplica mudanças à imagem local (Nota: você precisa reiniciar o sistema para que as mudanças sejam aplicadas)
- `sudo abroot rollback` : reverte o sistema para o estado anterior
- `sudo abroot kargs edit|show` : edita/Mostra os parâmetros do kernel
- `sudo abroot status` : mostra o estado
- `abroot -h` : mostra ajuda

## Links
- More information: https://docs.vanillaos.org/docs/en/abroot-manpage
