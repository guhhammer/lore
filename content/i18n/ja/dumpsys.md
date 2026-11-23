# dumpsys

Android システムサービスの情報を取得します。 このコマンドは `adb shell` 経由でのみ実行できます。

## Usage
- `dumpsys` : すべてのシステムに対して診断を出力します
- `dumpsys service` : 特定のシステムに対して診断を出力します
- `dumpsys -l` : `dumpsys` で表示できるすべてのサービスの情報を表示します
- `dumpsys service -h` : 指定したサービスを引数をサービスに表示させます
- `dumpsys --skip service` : 指定したサービスを診断から除外して出力します
- `dumpsys -t 8` : タイムアウトを秒数で指定します。(デフォルトは10秒)

## Links
- More information: https://developer.android.com/tools/dumpsys
