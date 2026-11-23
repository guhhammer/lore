# git difftool

外部diffツールを使ってファイルの変更を表示する。`git diff` と同じオプションと引数を受け付ける。

## Usage
- `git difftool --tool-help` : 利用可能なdiffツールを一覧表示する
- `git config --global diff.tool "meld"` : 既定のdiffツールをMeldに設定する
- `git difftool --staged` : 既定のdiffツールを使ってステージされた変更を表示する
- `git difftool -t opendiff コミット` : 指定したツールを使って、指定したコミット以降の変更を表示する

## Links
- More information: https://git-scm.com/docs/git-difftool
