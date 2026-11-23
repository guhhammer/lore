# snap.pkg

自己完結型の "snap" ソフトウェアパッケージを管理します。 `apt` が `.deb` の為のものであるのと似ています。

## Usage
- `snap find クエリ` : パッケージを検索する
- `snap install パッケージ` : パッケージをインストール
- `snap refresh パッケージ` : パッケージを更新
- `snap refresh パッケージ --channel=チャンネル` : パッケージを別のチャンネルに更新 (track, risk, または branch)
- `snap refresh` : 全てのパッケージを更新する
- `snap list` : インストールされている snap ソフトウェアの基本情報を表示
- `snap remove パッケージ` : パッケージをアンインストール
- `snap changes` : システムに最近の snap の変更がないか確認

## Links
- More information: https://manned.org/snap
