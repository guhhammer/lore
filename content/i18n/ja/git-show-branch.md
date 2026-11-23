# git show-branch

ブランチとそのコミットを表示する。

## Usage
- `git show-branch ブランチ名|参照|コミット` : ブランチの最新コミットの概要を表示する
- `git show-branch ブランチ名1|参照1|コミット1 ブランチ名2|参照2|コミット2 ...` : 複数のコミットまたはブランチの履歴にあるコミットを比較する
- `git show-branch -r` : すべてのリモート追跡ブランチを比較する
- `git show-branch -a` : ローカルとリモートの両方の追跡ブランチを比較する
- `git show-branch -a --list` : すべてのブランチの最新コミットを一覧表示する
- `git show-branch --current コミット|ブランチ名|参照` : 指定したブランチと現在のブランチを比較する
- `git show-branch --sha1-name --current 現在|ブランチ名|参照` : 相対名の代わりにコミット名を表示する
- `git show-branch --more 5 ブランチ名1|参照1|コミット1 ブランチ名2|参照2|コミット2 ...` : 共通の祖先より前のコミットを指定した数だけ表示する

## Links
- More information: https://git-scm.com/docs/git-show-branch
