# gpg

GNU Privacy Guard。

## Usage
- `gpg --full-gen-key` : GPGの公開鍵と秘密鍵を対話的に作成する
- `gpg --clearsign doc.txt` : 暗号化せずに `doc.txt` に署名する (出力を `doc.txt.asc` に書き出す)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : alice@example.com と bob@example.com の `doc.txt` を暗号化して署名する (`doc.txt.gpg` に出力)
- `gpg -c doc.txt` : パスフレーズのみで `doc.txt` を暗号化する (`doc.txt.gpg` に出力)
- `gpg -d doc.txt.gpg` : `doc.txt.gpg` を復号化する (`stdout` に出力)
- `gpg --import public.gpg` : 公開鍵をインポートする
- `gpg --export -a alice@example.com` : alice@example.com 用に公開鍵をエクスポートする (`stdout` に出力)
- `gpg --export-secret-keys -a alice@example.com` : alice@example.com の秘密鍵をエクスポートする (`stdout` に出力)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
