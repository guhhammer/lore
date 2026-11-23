# git apply

コミットを作成せず、ファイルやインデックスにパッチを適用する。

## Usage
- `git apply -v ディレクトリ/サブディレクトリ/ファイル` : パッチを適用したファイルに関するメッセージを表示する
- `git apply --index ディレクトリ/サブディレクトリ/ファイル` : パッチを適用し、変更したファイルをインデックスに追加する
- `curl -L https://example.com/file.patch | git apply` : リモートのパッチファイルを適用する
- `git apply --stat --apply ディレクトリ/サブディレクトリ/ファイル` : 入力の差分統計を出力してからパッチを適用する
- `git apply -R ディレクトリ/サブディレクトリ/ファイル` : パッチを逆向きに適用する
- `git apply --cached ディレクトリ/サブディレクトリ/ファイル` : 作業ツリーを変更せず、パッチの適用結果をインデックスに保存する

## Links
- More information: https://git-scm.com/docs/git-apply
