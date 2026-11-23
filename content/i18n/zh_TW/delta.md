# delta

Git 和 diff 輸出的檢視器。

## Usage
- `delta 路徑/至/舊檔案或目錄 路徑/至/新檔案或目錄` : 比較檔案或目錄
- `delta -n 路徑/至/舊檔案或目錄 路徑/至/新檔案或目錄` : 比較檔案或目錄，顯示行號
- `delta -s 路徑/至/舊檔案或目錄 路徑/至/新檔案或目錄` : 比較檔案或目錄，並排顯示差異
- `delta --no-gitconfig 路徑/至/舊檔案或目錄 路徑/至/新檔案或目錄` : 比較檔案或目錄，忽略任何 Git 配置設定
- `delta --hyperlinks 路徑/至/舊檔案或目錄 路徑/至/新檔案或目錄` : 比較，根據終端模擬器的超連結規範，將提交雜湊、檔案名稱和行號渲染為超連結
- `delta --show-config` : 顯示目前設定
- `delta --list-languages` : 顯示支援的語言及相關檔案副檔名

## Links
- More information: https://dandavison.github.io/delta/full---help-output.html
