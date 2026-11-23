# more

`stdin` またはファイルからのページ分割された出力を表示します。

## Usage
- `echo test | more` : `stdin`の出力をページ分割して表示する
- `more path\to\file` : 1つ以上のファイルからページ分割された出力を表示する
- `more path\to\file /tスペース数` : タブを指定した数のスペースに変換する
- `more path\to\file /c` : ページを表示する前に画面をクリアする
- `more path\to\file +5` : 5行目からの出力を表示する
- `more path\to\file /e` : 拡張インタラクティブモードを有効にする(使い方はヘルプを参照)
- `more /?` : ヘルプを表示する
