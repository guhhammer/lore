# input

イベントコードまたはタッチスクリーンジェスチャーをAndroidデバイスに送信します。 このコマンドは `adb shell` 経由でのみ実行できます。

## Usage
- `input keyevent event_code` : androidデバイスに1文字のイベントコードを送信します
- `input text "text"` : androidデバイスにテキストを送信します。(`%s` はスペースを表します)
- `input tap x_position y_position` : androidデバイスに単一のタップを送信します
- `input swipe x_start y_start x_end y_end duration_in_ms` : android デバイスにスワイプジェスチャーを送信します
- `input swipe x_position y_position x_position y_position duration_in_ms` : androidデバイスにホールドをスワイプジェスチャーを利用して送信します
