# git restore

作業ツリーのファイルを復元する。Git 2.23以降が必要。

## Usage
- `git restore ディレクトリ/サブディレクトリ/ファイル` : ステージされていないファイルを、ステージ済みのバージョンへ復元する
- `git restore -s コミット ディレクトリ/サブディレクトリ/ファイル` : ステージされていないファイルを、指定したコミットのバージョンへ復元する
- `git restore :/` : 追跡済みファイルのステージされていない変更をすべて破棄する
- `git restore -S ディレクトリ/サブディレクトリ/ファイル` : ファイルのステージを解除する
- `git restore -S :/` : すべてのファイルのステージを解除する
- `git restore -W -S :/` : ステージ済みかどうかにかかわらず、ファイルへの変更をすべて破棄する
- `git restore -p` : 復元するファイルの部分を対話形式で選択する

## Links
- More information: https://git-scm.com/docs/git-restore
