# flatpak

Flatpak アプリケーションとランタイムのビルド、インストール、実行をします。

## Usage
- `flatpak run com.example.app` : インストール済のアプリケーションを実行する
- `flatpak install リモート名 com.example.app` : リモートソースからアプリケーションをインストール
- `flatpak list --app` : ランタイムは無視して、インストールされたアプリケーションを一覧表示
- `flatpak update` : インストールされている全てのアプリケーションとランタイムを更新
- `flatpak remote-add --if-not-exists リモート名 リモートurl` : リモートソースを追加
- `flatpak remove com.example.app` : インストール済のアプリケーションを削除
- `flatpak remove --unused` : 未使用のアプリケーションを全て削除
- `flatpak info com.example.app` : インストールされているアプリケーションの情報を表示

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak
