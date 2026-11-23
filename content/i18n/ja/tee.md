# tee

`stdin` から読み込んで `stdout` とファイル(またはコマンド)に書き込みます。

## Usage
- `echo "example" | tee path/to/file` : 各ファイルに `stdin` をコピーし、`stdout` にもコピーする
- `echo "example" | tee -a path/to/file` : 与えられたファイルに追記する。上書きはしない
- `echo "example" | tee /dev/tty | xargs printf "[%s]"` : ターミナルに `stdin` を表示し、さらに処理するために別のプログラムにパイプする
- `echo "example" | tee >(xargs mkdir) >(wc -c)` : "example"と言うディレクトリを作成し、"example"の文字バイト数を数え、"example"をターミナルに出力する

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tee-invocation.html
