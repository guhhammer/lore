# npm

JavaScript と Node.js のパッケージマネージャ。 Node.js プロジェクトとそのモジュールの依存関係を管理します。

## Usage
- `npm init -y` : デフォルト値の `package.json` ファイルを生成 (対話的に行う場合は `--yes` を省略)
- `npm i` : `package.json`に依存パッケージとしてリストされている全てのパッケージをダウンロード
- `npm i パッケージ名@バージョン` : パッケージの特定のバージョンをダウンロードして `package.json` の dependencies に追加
- `npm i パッケージ名 -D` : パッケージの最新バージョンをダウンロードして `package.json` の dev dependencies に追加
- `npm i パッケージ名 -g` : パッケージの最新バージョンをダウンロードし、グローバルにインストール
- `npm r パッケージ名` : パッケージをアンインストールして `package.json` の dependencies から削除
- `npm ls` : ローカルにインストールされている依存パッケージを全て一覧表示
- `npm ls -g --depth 0` : グローバルにインストールされているトップレベルのパッケージを一覧表示する

## Links
- More information: https://docs.npmjs.com/cli/npm/
