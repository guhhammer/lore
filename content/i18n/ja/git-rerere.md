# git rerere

マージコンフリクトの解決内容を再利用する。

## Usage
- `git config --global rerere.enabled true` : rerereをグローバルに有効化する
- `git rerere forget ディレクトリ/サブディレクトリ/ファイル` : ファイルに記録された解決内容を忘れる
- `git rerere status` : 記録された解決内容の状態を確認する

## Links
- More information: https://git-scm.com/docs/git-rerere
