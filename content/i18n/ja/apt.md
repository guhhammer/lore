# apt

Debian系ディストリビューションで使われるパッケージ管理システムです。 Ubuntuのバージョンが16.04か、それ以降で対話モードを使う場合`apt-get`の代わりとして使用します。

## Usage
- `sudo apt update` : 利用可能なパーケージとバージョンのリストの更新（他の`apt`コマンドの前での実行を推奨）
- `apt search パッケージ` : 指定されたパッケージの検索
- `apt show パッケージ` : パッケージの情報を出力
- `sudo apt install パッケージ` : パッケージのインストール、または利用可能な最新バージョンに更新
- `sudo apt remove パッケージ` : パッケージの削除（`sudo apt remove --purge`の場合設定ファイルも削除）
- `sudo apt upgrade` : インストールされている全てのパッケージを最新のバージョンにアップグレード
- `apt list` : インストールできるすべてのパッケージを表示
- `apt list -i` : インストールされた全てのパッケージを表示（依存関係も表示）

## Links
- More information: https://manned.org/apt.8
