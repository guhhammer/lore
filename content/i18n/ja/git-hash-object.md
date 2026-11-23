# git hash-object

内容の一意なハッシュキーを計算し、必要に応じて指定した種類のオブジェクトを作成する。

## Usage
- `git hash-object ディレクトリ/サブディレクトリ/ファイル` : オブジェクトIDを保存せずに計算する
- `git hash-object -w ディレクトリ/サブディレクトリ/ファイル` : オブジェクトIDを計算し、Gitデータベースに保存する
- `git hash-object -t blob|commit|tag|tree ディレクトリ/サブディレクトリ/ファイル` : オブジェクトの種類を指定してオブジェクトIDを計算する
- `cat ディレクトリ/サブディレクトリ/ファイル | git hash-object --stdin` : `stdin` からオブジェクトIDを計算する

## Links
- More information: https://git-scm.com/docs/git-hash-object
