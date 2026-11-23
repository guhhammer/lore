# git fast-export

Gitリポジトリの内容と履歴を、ストリーミング可能なプレーンテキスト形式でエクスポートする。

## Usage
- `git fast-export --all` : リポジトリの履歴全体を `stdout` へエクスポートする
- `git fast-export --all >ディレクトリ/サブディレクトリ/ファイル` : リポジトリ全体をファイルへエクスポートする
- `git fast-export main` : 指定したブランチだけをエクスポートする
- `git fast-export --progress n --all >ディレクトリ/サブディレクトリ/ファイル` : `n` 個のオブジェクトごとに `progress` 文を出力する (`git fast-import` の進捗表示用)
- `git fast-export --all -- ディレクトリ/サブディレクトリ >ディレクトリ/サブディレクトリ/ファイル` : 指定したサブディレクトリの履歴だけをエクスポートする

## Links
- More information: https://manned.org/git-fast-export
