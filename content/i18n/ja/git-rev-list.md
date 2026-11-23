# git rev-list

リビジョン (コミット) を新しい順に一覧表示する。

## Usage
- `git rev-list HEAD` : 現在のブランチ上のすべてのコミットを一覧表示する
- `git rev-list -n 1 HEAD -- ディレクトリ/サブディレクトリ/ファイル` : 現在のブランチで指定したファイルを変更 (追加、編集、削除) した最新コミットを出力する
- `git rev-list --since "2019-12-01 00:00:00" ブランチ名` : 指定したブランチで、指定した日付より新しいコミットを一覧表示する
- `git rev-list --merges コミット` : 指定したコミット上のすべてのマージコミットを一覧表示する
- `git rev-list タグ名..HEAD --count` : 指定したタグ以降のコミット数を出力する

## Links
- More information: https://git-scm.com/docs/git-rev-list
