# nvim

Neovim, um editor de texto para programadores baseado no Vim, oferece vários modos para diferentes tipos de manipulação de texto. Pressionar`<i>` no modo normal entra no modo de inserção. `<Esc>` retorna ao modo normal, que não permite a inserção regular de texto.

## Usage
- `nvim caminho/para/arquivo` : abre um arquivo
- `<Esc><i>` : entra no modo de edição de texto (mode de inserção)
- `<Esc><y><y>|<d><d>` : copia ("yank") ou recorta ("delete") a linha atual (cole-a com `<p>`)
- `<Esc><u>` : entra no modo normal e desfaz a última operação
- `<Esc></>padrão_procurado<Enter>` : procura por um padrão em um arquivo (pressione `<n>`/`<N>` para ir para a próxima/prévia correspondência)
- `<Esc><:>%s/expressão_regular/substituição/g<Enter>` : executa uma substituição de expressão regular em todo o arquivo
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : entra no modo normal, salva (grava) o arquivo e sai
- `<Esc><:>q!<Enter>` : sai sem salvar

## Links
- More information: https://neovim.io/
