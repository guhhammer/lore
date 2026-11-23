# usql

通用 SQL 数据库的命令行界面。

## Usage
- `usql sqlserver|mysql|postgres|sqlite3|...://用户名:密码@主机:端口/数据库名` : 连接到一个特定的数据库
- `usql -f 路径/到/文件.sql` : 从文件中执行命令
- `usql -c "sql命令"` : 执行一个特定的 SQL 命令
- `sql命令` : [交互式] 在 `usql` 提示符下运行一个 SQL 命令
- `\d` : [交互式] 显示数据库架构
- `\g 路径/到/结果文件` : [交互式] 将查询结果导出到一个特定文件
- `\copy 路径/到/data.csv 表名` : [交互式] 从 CSV 文件导入数据到一个特定表

## Links
- More information: https://github.com/xo/usql#using
