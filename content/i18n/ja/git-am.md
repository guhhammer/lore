# git am

パッチファイルを適用してコミットを作成する。メールでコミットを受け取る場合に便利。

## Usage
- `git am ディレクトリ/サブディレクトリ/ファイル.patch` : ローカルのパッチファイルに従って変更を適用し、コミットする
- `curl -L https://example.com/file.patch | git am` : リモートのパッチファイルに従って変更を適用し、コミットする
- `git am --abort` : パッチファイルの適用処理を中止する
- `git am --reject ディレクトリ/サブディレクトリ/ファイル.patch` : パッチファイルを可能な限り適用し、失敗したハンクをrejectファイルに保存する

## Links
- More information: https://git-scm.com/docs/git-am
