# git ls-files

インデックスと作業ツリー内のファイル情報を表示する。

## Usage
- `git ls-files -d` : 削除されたファイルを表示する
- `git ls-files -m` : 変更および削除されたファイルを表示する
- `git ls-files -o` : `.gitignore` などで無視されているファイルも含めて、未追跡ファイルを表示する
- `git ls-files -o --exclude-standard` : `.gitignore` などで無視されているファイルを除き、未追跡ファイルを表示する

## Links
- More information: https://git-scm.com/docs/git-ls-files
