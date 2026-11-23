# nginx

`nginx` ウェブサーバ。

## Usage
- `nginx` : デフォルトの設定ファイルでサーバーを起動する
- `nginx -c 設定ファイル` : カスタム設定ファイルでサーバを起動する
- `nginx -c 設定ファイル -p prefix/for/relative/paths` : 設定ファイル内の全ての相対パスのプレフィクスを指定して、サーバを起動する
- `nginx -t` : 実行中のサーバに影響を与えずに設定をテストする
- `nginx -s reload` : ダウンタイムなしでシグナルを送って、設定をリロードする

## Links
- More information: https://nginx.org/docs/switches.html
