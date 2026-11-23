# git merge

ブランチをマージする。

## Usage
- `git merge ブランチ名1 ブランチ名2 ...` : 指定したブランチを現在のブランチへマージする
- `git merge -e ブランチ名` : マージメッセージを編集する
- `git merge --no-ff ブランチ名` : ブランチをマージし、マージコミットを作成する
- `git merge --squash ブランチ名` : コミットを作成せずに、ブランチのマージ結果をステージする
- `git merge --abort` : コンフリクトが発生したマージを中止する
- `git merge -s 戦略 -X 戦略オプション ブランチ名` : 指定した戦略を使ってマージする

## Links
- More information: https://git-scm.com/docs/git-merge
