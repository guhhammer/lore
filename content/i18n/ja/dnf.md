# dnf

RHEL、Fedora、CentOS 用のパッケージ管理ユーティリティ (yum の後継)。 `group`, `config-manager` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `sudo dnf upgrade` : インストールされたパッケージを、利用可能な最新バージョンにアップグレードする
- `dnf search キーワード1 キーワード2 ...` : キーワードでパッケージを検索
- `dnf info パッケージ` : パッケージの詳細を表示
- `sudo dnf install パッケージ1 パッケージ2 ...` : 新しいパッケージをインストール (`-y` を使用すると、全てのプロンプトを自動的に追認する)
- `sudo dnf remove パッケージ1 パッケージ2 ...` : パッケージを削除
- `dnf list --installed` : インストールされているパッケージを一覧表示
- `dnf provides コマンド` : 指定したコマンドを提供するパッケージを検索
- `dnf history` : 過去の全オペレーションを見る

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
