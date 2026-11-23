# whatis

マニュアルページから、一行の説明文を表示します。

## Usage
- `whatis コマンド` : manページの説明文を表示する
- `whatis -l コマンド` : 説明文を行末で切らないで表示する
- `whatis -w net*` : `glob`パターンにマッチするすべてのコマンドの説明文を表示する
- `whatis -r 'wish[0-9]\.[0-9]'` : 正規表現でmanページの説明文を検索する
- `whatis -L ja コマンド` : 言語を指定して説明文で表示する

## Links
- More information: https://manned.org/whatis
