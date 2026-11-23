# node

サーバサイド JavaScript プラットフォーム (Node.js)。

## Usage
- `node path/to/file` : javaScript ファイルを実行
- `node` : REPL(インタラクティブシェル)を開始
- `node --watch path/to/file` : インポートされたファイルが変更された時に、プロセスを再起動して指定されたファイルを実行する (Node.js version 18.11+ が必要)
- `node -e "code"` : javaScript コードを引数として渡して評価する
- `node -p "process.versions"` : 結果を評価して表示する。ノードの依存関係のバージョンを出力するのに役立つ
- `node --no-lazy --inspect-brk path/to/file` : ソースコードが完全に解析されたら、デバッガが接続されるまで実行を一時停止するインスペクタをアクティブにする

## Links
- More information: https://nodejs.org/docs/latest/api/cli.html#options
