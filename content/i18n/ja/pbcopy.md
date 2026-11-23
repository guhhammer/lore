# pbcopy

`stdin`(標準入力)からデータをクリップボードにコピーします。 これは、キーボードの`<Cmd c>`を押す操作に相当します。

## Usage
- `pbcopy <パス/宛先/ファイル` : 指定したファイルの内容をクリップボードにコピーします
- `find . -type t -name "*.png" | pbcopy` : 特定のコマンドの実行結果をクリップボードにコピーします

## Links
- More information: https://keith.github.io/xcode-man-pages/pbcopy.1.html
