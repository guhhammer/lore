# trash

管理回收站。

## Usage
- `trash 路径/到/文件` : 将文件移入回收站
- `trash-list` : 列出回收站中的所有文件
- `trash-restore` : 以交互方式从回收站恢复文件
- `trash-empty` : 清空回收站
- `trash-empty 10` : 永久删除回收站中超过 10 天的所有文件
- `trash-rm "*.o"` : 删除回收站中匹配指定 `glob` 模式的所有文件
- `trash-rm /路径/到/文件或目录` : 删除具有指定原始位置的所有文件

## Links
- More information: https://github.com/andreafrancia/trash-cli
