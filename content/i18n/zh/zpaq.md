# zpaq

增量日志式备份工具和归档程序。

## Usage
- `zpaq a 路径/到/归档.zpaq 路径/到/文件或目录` : 将文件或目录添加到新的或已有的归档中
- `zpaq a -k密码 路径/到/归档.zpaq 路径/到/文件或目录` : 创建或添加到加密的归档
- `zpaq x 路径/到/归档.zpaq` : 解压文件的最新版本
- `zpaq l 路径/到/归档.zpaq` : 列出归档内容
- `zpaq a 路径/到/归档.zpaq -m1..5 路径/到/文件或目录` : 设置压缩级别（级别越高，压缩率越高但速度越慢）
- `zpaq x 路径/到/归档.zpaq 归档/中/要/解压/的/路径 -to 路径/到/输出 -until YYYY-MM-DD` : 从归档中解压不晚于指定日期的指定文件

## Links
- More information: https://mattmahoney.net/dc/zpaqdoc.html
