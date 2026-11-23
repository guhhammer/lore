# mysql

MySQL 命令列工具。

## Usage
- `mysql 資料庫名稱` : 與資料庫連線
- `mysql -u 使用者名稱 -p 資料庫名稱` : 與資料庫連線，系統將提示使用者輸入密碼
- `mysql -h 資料庫主機 資料庫名稱` : 連線到另一台主機上的資料庫
- `mysql -S sock 檔路徑` : 透過 Unix 通訊端連接到資料庫
- `mysql -e "source sql 檔案" 資料庫名稱` : 執行腳本檔案（批次檔）中的 `SQL` 語句
- `mysql <sql_備份檔路徑 -u 使用者名稱 -p 資料庫名稱` : 用 `mysqldump` 建立的備份還原資料庫（系統將提示使用者輸入密碼）
- `mysql <sql_備份檔路徑 -u 使用者名稱 -p` : 從備份中恢復所有資料庫（系統將提示使用者輸入密碼）

## Links
- More information: https://manned.org/mysql
