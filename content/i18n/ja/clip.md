# clip

入力コンテンツをWindowsクリップボードにコピーします。

## Usage
- `dir | clip` : コマンドライン出力をWindowsクリップボードにパイプします
- `clip <ファイルパス.ext` : ファイルの内容をWindowsクリップボードにコピーします
- `echo テキスト | clip` : 末尾に改行が付いたテキストをWindowsクリップボードにコピーします
- `echo | set /p="テキスト" | clip` : 末尾の改行なしでテキストをWindowsクリップボードにコピーします
