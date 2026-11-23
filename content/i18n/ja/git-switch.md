# git switch

Gitブランチを切り替える。Git 2.23以降が必要。

## Usage
- `git switch ブランチ名` : 既存のブランチに切り替える
- `git switch -c ブランチ名` : 新しいブランチを作成して切り替える
- `git switch -c ブランチ名 コミット` : 既存のコミットを基に新しいブランチを作成して切り替える
- `git switch -` : 直前のブランチに切り替える
- `git switch --recurse-submodules ブランチ名` : ブランチへ切り替え、すべてのサブモジュールも一致するように更新する
- `git switch -m ブランチ名` : ブランチへ切り替え、現在のブランチと未コミットの変更を自動的にマージする
- `git switch -d タグ|コミット` : タグまたはコミットに切り替える
- `git switch -t リモート名/ブランチ` : 別のリモートにあるブランチへ切り替える

## Links
- More information: https://git-scm.com/docs/git-switch
