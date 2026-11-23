# emacs

O editor extensível, personalizável, autodocumentável, com exibição em tempo real.

## Usage
- `emacs caminho/para/arquivo` : inicia o Emacs e abra um arquivo
- `emacs +numero_linha caminho/para/arquivo` : abre um arquivo em uma linha especificada
- `emacs --script caminho/para/arquivo.el` : inicia um arquivo Emacs Lisp como script
- `emacs -nw` : inicia o Emacs em modo console (sem uma janela X)
- `emacs --daemon` : inicia um servidor Emacs em segundo plano (acessível através do `emacsclient`)
- `emacsclient --eval '(save-buffers-kill-emacs)'` : para um servidor Emacs em funcionamento e todas as suas instâncias, pedindo confirmação em arquivos não salvos
- `<Ctrl x><Ctrl s>` : salva um arquivo em Emacs
- `<Ctrl x><Ctrl c>` : sai do Emacs

## Links
- More information: https://www.gnu.org/software/emacs
