# git bundle

オブジェクトと参照をアーカイブにまとめる。

## Usage
- `git bundle create ディレクトリ/サブディレクトリ/ファイル.bundle ブランチ名` : 指定したブランチのすべてのオブジェクトと参照を含むバンドルファイルを作成する
- `git bundle create ディレクトリ/サブディレクトリ/ファイル.bundle --all` : すべてのブランチのバンドルファイルを作成する
- `git bundle create ディレクトリ/サブディレクトリ/ファイル.bundle -5 HEAD` : 現在のブランチの直近5コミット分のバンドルファイルを作成する
- `git bundle create ディレクトリ/サブディレクトリ/ファイル.bundle --since 7.days HEAD` : 直近7日分のバンドルファイルを作成する
- `git bundle verify ディレクトリ/サブディレクトリ/ファイル.bundle` : バンドルファイルが有効で、現在のリポジトリに適用できることを検証する
- `git bundle unbundle ディレクトリ/サブディレクトリ/ファイル.bundle` : バンドルに含まれる参照の一覧を `stdout` に出力する
- `git pull ディレクトリ/サブディレクトリ/ファイル.bundle ブランチ名` : バンドルファイルから指定したブランチを現在のリポジトリへ展開する
- `git clone ディレクトリ/サブディレクトリ/ファイル.bundle` : バンドルから新しいリポジトリを作成する

## Links
- More information: https://git-scm.com/docs/git-bundle
