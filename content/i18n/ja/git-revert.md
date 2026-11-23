# git revert

以前のコミットの効果を打ち消す新しいコミットを作成する。

## Usage
- `git revert HEAD` : 最新のコミットを打ち消す
- `git revert HEAD~4` : 最新から数えて5番目のコミットを打ち消す
- `git revert 0c01a9` : 指定したコミットを打ち消す
- `git revert ブランチ名~5..ブランチ名~2` : 複数のコミットを打ち消す
- `git revert -n 0c01a9..9a1743` : 新しいコミットを作成せず、作業ツリーだけを変更する
- `git revert --abort` : マージコンフリクト後に `git revert` を中止する

## Links
- More information: https://git-scm.com/docs/git-revert
