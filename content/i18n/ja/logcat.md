# logcat

エラー時に発生したスタックトレースを含む、アプリケーションによって作成されたログや、システムメッセージのログを削除します。

## Usage
- `logcat` : システムログを表示します
- `logcat -f path/to/file` : 指定したファイルにシステムログを書き出します
- `logcat --regex regular_expression` : 正規表現にマッチする行を表示します
- `logcat --pid pid` : 指定したPIDのログを表示します
- `logcat --pid $(pidof -s package)` : 特定のパッケージのプロセスのログを表示します

## Links
- More information: https://developer.android.com/tools/logcat
