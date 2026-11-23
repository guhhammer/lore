# git merge-index

マージが必要なファイルに対してマージプログラムを実行する。

## Usage
- `git merge-index git-merge-one-file -a` : 標準ヘルパーを使用して、解決が必要なすべてのファイルをマージする
- `git merge-index git-merge-one-file -- ディレクトリ/サブディレクトリ/ファイル` : 指定したファイルをマージする
- `git merge-index -o git-merge-one-file -- ディレクトリ/サブディレクトリ/ファイル1 ディレクトリ/サブディレクトリ/ファイル2 ...` : 複数のファイルをマージし、失敗しても処理を続行する
- `git merge-index -q マージプログラム -a` : カスタムプログラムを使用し、出力を抑制してすべてのファイルをマージする
- `git merge-index cat -- パス` : `cat` を使用して、ファイルのマージ入力を確認する

## Links
- More information: https://git-scm.com/docs/git-merge-index
