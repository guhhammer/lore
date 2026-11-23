# winget

Windows パッケージマネージャ。

## Usage
- `winget add パッケージ` : パッケージをインストール
- `winget rm パッケージ` : パッケージを削除 (注意: `uninstall` の代わりに `remove` を使用することもできる)
- `winget show パッケージ` : パッケージに関する情報を表示
- `winget search パッケージ` : パッケージを検索
- `winget upgrade -r` : 全てのパッケージを最新バージョンにアップグレード
- `winget ls -s winget` : `winget`で管理可能なインストール済パッケージを全て一覧表示
- `winget import|export --import-file|--output ファイルパス` : ファイルからのパッケージのインポート、またはインストールされたパッケージのファイルへのエクスポート
- `winget validate 明示パス` : winget-pkgs リポジトリに PR を開く前にマニフェストを検証する

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
