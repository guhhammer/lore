# mongod

MongoDB 数据库服务器。

## Usage
- `mongod --dbpath 路径/到/目录` : 指定存储目录（默认：Linux 和 macOS 为 `/data/db`，Windows 为 `C:\data\db`）
- `mongod --config 路径/到/文件` : 指定配置文件
- `mongod --port 端口` : 指定监听的端口（默认：27017）
- `mongod --profile 0|1|2` : 指定数据库分析级别。0 为关闭，1 仅慢操作，2 所有（默认：0）

## Links
- More information: https://www.mongodb.com/docs/manual/reference/program/mongod/
