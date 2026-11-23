# diff

ファイルとディレクトリを比較する。

## Usage
- `diff old_file new_file` : ファイルを比較する(`old_file`を`new_file`にするための変更点を列挙する)
- `diff -w old_file new_file` : 空白を無視してファイルを比較する
- `diff -y old_file new_file` : ファイルを比較し、差分を並べて表示する
- `diff -u old_file new_file` : ファイルを比較し、差分を統一フォーマットで表示する(`git diff`で使用される)
- `diff -r old_directory new_directory` : ディレクトリを再帰的に比較する (異なるファイル/ディレクトリの名前と、ファイルに加えられた変更を表示します)
- `diff -r -q old_directory new_directory` : ディレクトリを比較し、異なるファイル名のみを表示する
- `diff -a -u -N old_file new_file >diff.patch` : 2つのテキストファイルの差分からGit用のパッチファイルを作成します。存在しないファイルは空ファイルとして扱います
- `diff -d --color=always old_file new_file` : ファイルを比較し、出力を色分けして表示する

## Links
- More information: https://manned.org/diff
