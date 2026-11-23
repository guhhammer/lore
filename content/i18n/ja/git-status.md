# git status

Git リポジトリ内のファイルの変更状況を表示する。 現在チェックアウトされているコミットと比較して、変更・追加・削除されたファイルを一覧表示する。

## Usage
- `git status` : コミット対象にまだ追加されていない未追跡ファイルと変更済みファイルを表示する
- `git status -s` : 短い形式で表示する
- `git status -sb` : 短い形式でブランチ情報を表示する
- `git status -v` : ステージング済みファイルの変更内容を表示する (`git diff --cached` と同様)
- `git status -vv` : 追跡済みのすべてのファイルの変更内容を表示する (`git diff HEAD` と同様)
- `git status --show-stash` : 現在スタッシュに保存されているエントリ数を表示する
- `git status -uno` : 未追跡ファイルを出力に表示しない

## Links
- More information: https://git-scm.com/docs/git-status
