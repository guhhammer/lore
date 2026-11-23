# git merge-base

2つのコミットのマージベースを見つける。

## Usage
- `git merge-base コミット1 コミット2` : 2つのコミットのマージベースを出力する
- `git merge-base -a コミット1 コミット2` : 2つのコミットのマージベースをすべて出力する
- `git merge-base --is-ancestor 対象コミット 基準コミット` : 指定したコミットが別のコミットの履歴上にあるか確認する

## Links
- More information: https://git-scm.com/docs/git-merge-base
