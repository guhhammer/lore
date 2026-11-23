# git p4

PerforceリポジトリからのインポートとPerforceリポジトリへの送信を行う。

## Usage
- `git p4 clone ディレクトリ/サブディレクトリ/p4_depot` : perforce depotを新しいGitリポジトリへクローンする
- `git p4 sync ディレクトリ/サブディレクトリ/p4_depot` : perforceからの変更を現在のGitリポジトリへ同期する
- `git p4 rebase` : ローカルコミットを最新のPerforce変更の上にリベースする
- `git p4 submit` : gitの変更をPerforceへ送信する
- `git p4 clone ディレクトリ/サブディレクトリ/p4_depot@all` : 最新のchangelistだけでなく、Perforceの履歴全体をクローンする

## Links
- More information: https://git-scm.com/docs/git-p4
