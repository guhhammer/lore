# alias

alias （コマンド文字列を置き換える別名）を作成します。 alias は `~/.bashrc` などの shell の設定ファイルで定義されない限り、現在のセッションで失効します。

## Usage
- `alias` : alias の一覧表示
- `alias 別名="コマンド"` : alias を作成する
- `alias 別名` : 指定した alias に紐付くコマンドの表示
- `unalias 別名` : 作成された alias の削除
- `alias rm="rm -i"` : `rm` を対話型にする
- `alias la="ls -a"` : `ls -a` のショートカットして `la` を作成する

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
