# ccomps

将图形分解为它们的连通分量。

## Usage
- `ccomps 路径/到/输入1.gv 路径/到/输入2.gv ... >路径/到/输出.gv` : 将一个或多个图形分解为它们的连通分量
- `ccomps -v -s 路径/到/输入1.gv 路径/到/输入2.gv ...` : 打印一个或多个图形中的节点、边和连通分量的数量
- `ccomps -x -o 路径/到/输出.gv 路径/到/输入1.gv 路径/到/输入2.gv ...` : 根据 `输出.gv` 将每个连通分量写入多个有编号的文件中
- `ccomps -?` : 显示帮助

## Links
- More information: https://graphviz.org/pdf/ccomps.1.pdf
