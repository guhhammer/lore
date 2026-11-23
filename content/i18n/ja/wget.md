# wget

Webからファイルをダウンロードします。 HTTP, HTTPS, そして FTP をサポートします。

## Usage
- `wget https://example.com/foo` : URLの内容を、ファイルにダウンロードする (この場合 "foo" と言う名前で)
- `wget -O bar https://example.com/foo` : URLの内容を、ファイルにダウンロードする (この場合 "bar" と言う名前で)
- `wget -pkw 3 https://example.com/somepage.html` : 1つのウェブページと、その全てのリソースを、リクエスト間隔を3秒にしてダウンロードする (スクリプト、スタイルシート、画像など)
- `wget -mnp https://example.com/somepath/` : ディレクトリと、そのサブディレクトリ内のリストされたファイルを、全てダウンロードする (埋め込まれたページ要素はダウンロードしない)
- `wget --limit-rate 300k -t 100 https://example.com/somepath/` : ダウンロード速度と接続再試行回数を制限する
- `wget --user ユーザ名 --password パスワード https://example.com` : HTTPサーバーからBasic認証を使って、ファイルをダウンロードする (FTPも機能する)
- `wget -c https://example.com` : 未完了のダウンロードを続行する
- `wget -P path/to/directory -i URLs.txt` : テキストファイルに格納されている全てのURLを、特定のディレクトリにダウンロードする

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
