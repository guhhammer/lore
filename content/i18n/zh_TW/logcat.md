# logcat

轉存系統訊息日誌，包括發生錯誤時的堆疊追蹤信息，以及應用程序記錄的信息消息。

## Usage
- `logcat` : 顯示系統日誌
- `logcat -f 路徑/到/檔案` : 將系統日誌寫入檔案（file）
- `logcat --regex 正規表示式` : 顯示與`regex`匹配的日誌列
- `logcat --pid pid` : 顯示特定 PID 的日誌
- `logcat --pid $(pidof -s 套件)` : 顯示特定套件的程序日誌

## Links
- More information: https://developer.android.com/tools/logcat
