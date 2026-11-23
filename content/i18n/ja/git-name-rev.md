# git name-rev

既存の参照名を使ってコミットを説明する。

## Usage
- `git name-rev HEAD` : `HEAD` の名前を表示する
- `git name-rev --name-only HEAD` : 名前だけを表示する
- `git name-rev --all` : 一致するすべての参照名を列挙する
- `git name-rev --tags HEAD` : コミットの名前付けにタグだけを使用する
- `git name-rev --no-undefined コミットまたは参照` : 不明なコミットに対して `undefined` を出力する代わりに、0以外の終了ステータスで終了する
- `git name-rev HEAD~1 HEAD~2 main` : 複数のコミットの名前を表示する
- `git name-rev --refs refs/heads/ コミットまたは参照` : 名前をブランチ参照に限定する
- `echo "コミットまたは参照" | git name-rev --annotate-stdin` : `stdin` からコミットIDを読み取る

## Links
- More information: https://git-scm.com/docs/git-name-rev
