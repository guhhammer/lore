# git config

Gitリポジトリのカスタム設定を管理する。 設定はローカル (現在のリポジトリ用) またはグローバル (現在のユーザー用) にできる。

## Usage
- `git config --global user.name|user.email "あなたの名前|email@example.com"` : 名前またはメールアドレスをグローバルに設定する (この情報はリポジトリへのコミットに必要で、すべてのコミットに含まれる)
- `git config --local|global|system -l --show-origin` : ローカル、グローバル、またはシステムの設定項目と、その設定元ファイルを一覧表示する
- `git config --global alias.unstage "reset HEAD --"` : 指定した設定項目のグローバル値を設定する (この例ではエイリアス)
- `git config alias.unstage` : 指定した設定項目の値を取得する
- `git unstage` : エイリアスを使用する
- `git config --global --unset alias.unstage` : グローバル設定項目を既定値へ戻す
- `git config -e` : 既定のエディターでローカルGit設定 (`.git/config`) を編集する
- `git config --global -e` : 既定のエディターでグローバルGit設定 (既定では `~/.gitconfig`、存在する場合は `$XDG_CONFIG_HOME/git/config`) を編集する

## Links
- More information: https://git-scm.com/docs/git-config
