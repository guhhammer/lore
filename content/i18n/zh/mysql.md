# mysql

MySQL 命令行工具。

## Usage
- `mysql 数据库名` : 连接数据库
- `mysql -u 用户名 -p 数据库名` : 连接到数据库，系统将提示用户输入密码
- `mysql -h 数据库地址 数据库名` : 连接到另一台主机上的数据库
- `mysql -S 路径/到/socket.sock` : 通过 Unix 套接字文件连接到数据库
- `mysql -e "source 脚本.sql" 数据库名` : 执行脚本文件中的 SQL 语句
- `mysql <路径/到/备份文件.sql -u 用户名 -p 数据库名` : 从`mysqldump`创建的备份文件中恢复单个数据库（系统将提示用户输入密码）
- `mysql <路径/到/备份文件.sql -u 用户名 -p` : 从备份中恢复所有数据库（系统将提示用户输入密码）

## Links
- More information: https://manned.org/mysql
