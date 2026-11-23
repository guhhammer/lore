# top

即時顯示系統執行程序的資訊。

## Usage
- `top` : 啟動 `top`
- `top -i` : 不顯示閒置以及殭屍程序
- `top -u 使用者名稱` : 只顯示特定使用者之程序
- `top -o 領域名稱` : 依照指定領域排序
- `top -Hp 程序 id` : 查看程序底下的所有執行緒
- `top -p $(pgrep -d ',' 程序名稱)` : 僅顯示特定名稱程序的 PID
- `<?>` : 打開協助頁面

## Links
- More information: https://manned.org/top
