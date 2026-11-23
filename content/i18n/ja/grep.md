# grep

正規表現を使ってファイルのパターンを見つけます。

## Usage
- `grep "検索パターン" path/to/file` : ファイル内のパターンを検索する
- `grep -F "正確な文字列" path/to/file` : 正確な文字列を検索する（正規表現を無効にする）
- `grep -rnI "検索パターン" path/to/directory` : ディレクトリ内の全てのファイルを再帰的にパターン検索し、マッチしたファイルの行番号を表示する
- `grep -Ei "検索パターン" path/to/file` : 拡張正規表現 (`?`, `+`, `{}`, `()`, `|` をサポート)を大文字小文字を区別しないモードで使用する
- `grep --context|--before-context|--after-context 3 "検索パターン" path/to/file` : 各マッチの前後3行のコンテキストを表示する
- `grep -Hn --color=always "検索パターン" path/to/file` : 各マッチのファイル名と行番号をカラー出力する
- `grep -o "検索パターン" path/to/file` : パターンにマッチする行を検索し、マッチしたテキストのみを表示する
- `cat path/to/file | grep -v "検索パターン"` : パターンにマッチしない行を `stdin` から検索する

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
