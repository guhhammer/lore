# git mergetool

マージコンフリクト解決ツールを実行してマージコンフリクトを解決する。

## Usage
- `git mergetool` : 既定のマージツールを起動してコンフリクトを解決する
- `git mergetool --tool-help` : 有効なマージツールを一覧表示する
- `git mergetool -t ツール名` : 名前で指定したマージツールを起動する
- `git mergetool -y` : マージツールを起動するたびに確認しない
- `git mergetool -g` : GUIマージツールを明示的に使用する (`merge.guitool` 設定変数を参照)
- `git mergetool --no-gui` : 通常のマージツールを明示的に使用する (`merge.tool` 設定変数を参照)

## Links
- More information: https://git-scm.com/docs/git-mergetool
