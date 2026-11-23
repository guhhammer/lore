# rclone

在本地與多種雲端服務之間複製、同步或移動檔案和目錄。

## Usage
- `rclone config` : 啟動互動式選單來設定 rclone
- `rclone lsf 遠端名:路徑/至/目錄` : 列出 rclone 遠端目錄的內容
- `rclone copy 路徑/至/本地檔案或目錄 遠端名:路徑/至/目錄` : 將檔案或目錄從本地電腦複製到遠端目標
- `rclone copy -i --max-age 24h 遠端名:路徑/至/目錄 路徑/至/本地目錄` : 將過去 24 小時內修改過的檔案從本地複製到遠端，並向使用者逐個檔案確認
- `rclone sync 路徑/至/檔案或目錄 遠端名:路徑/至/目錄` : 鏡像特定的檔案或目錄（注意：與 copy 不同，sync 會將遠端中本地不存在的檔案刪除）
- `rclone -n delete 遠端名:路徑/至/檔案或目錄` : 刪除遠端檔案或目錄（注意：`--dry-run` 表示測試，從命令中移除它才會實際刪除）
- `rclone mount 遠端名:路徑/至/目錄 路徑/至/掛載點` : 掛載 rclone 遠端目錄（實驗性功能）
- `fusermount -u 路徑/至/掛載點` : 如果 `<Ctrl c>` 失敗，卸載 rclone 遠端目錄（實驗性功能）

## Links
- More information: https://rclone.org/commands/rclone/
