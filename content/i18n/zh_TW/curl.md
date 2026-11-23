# curl

向 / 從一個伺服器傳輸數據。 支持大多數協議，包括 HTTP、FTP 和 POP3。

## Usage
- `curl http://example.com -o 檔案/完整/路徑` : 將指定 URL 的內容下載到檔案
- `curl -O http://example.com/filename` : 將檔案從 URL 保存到由 URL 指示的檔名
- `curl -f -O -L -C - http://example.com/filename` : 下載檔案，跟隨重新導向，並且自動續傳（恢復）前序檔案傳輸
- `curl -d 'name=bob' http://example.com/form` : 發送表單編碼數據（`application/x-www-form-urlencoded` 的 POST 請求）
- `curl -H 'X-My-Header: 123' -X PUT http://example.com` : 發送帶有額外請求頭，使用自定義請求方法
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : 發送 JSON 格式的數據，並附加正確的 `Content-Type` 請求頭
- `curl -u 使用者名稱 http://example.com` : 透過使用者名稱和密碼訪問伺服器
- `curl -E client.pem --key key.pem -k https://example.com` : 爲指定資源使用客戶端憑證和密鑰，並且跳過憑證驗證

## Links
- More information: https://curl.se/docs/manpage.html
