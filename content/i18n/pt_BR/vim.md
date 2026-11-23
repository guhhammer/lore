# vim

Vim (Vi IMproved), é um editor de texto em linha de comando, que fornece muitos modos para diferentes tipos de manipulação de texto. Apertando `<i>` no modo normal entra em modo insert (inserir). Apertando `<Esc>` volta para o modo normal, que permite o uso dos comandos do Vim.

## Usage
- `vim caminho/para/arquivo` : abre um arquivo
- `vim +número_da_linha caminho/para/arquivo` : abre um arquivo em um número da linha específica
- `<:>help<Enter>` : abre o manual do Vim em visualização
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : salva e sai do arquivo atual
- `<Esc><u>` : entra em modo normal e desfaz a última operação
- `</>sequência_padrão_procurada<Enter>` : procura por um sequência padrão dentro de um arquivo (aperte `<n>`/`<N>` para ir para próxima/anterior sequência padrão)
- `<:>%s/expressão_regular/substituição/g<Enter>` : executa uma substituição por expressão regular no arquivo todo
- `<:>set nu<Enter>` : mostra os números das linhas

## Links
- More information: https://www.vim.org/
