# bcomps

将图形分解为它们的双连通分量。

## Usage
- `bcomps 路径/到/输入1.gv 路径/到/输入2.gv ... >路径/到/输出.gv` : 将一个或多个图形分解为它们的双连通分量
- `bcomps -v -s 路径/到/输入1.gv 路径/到/输入2.gv ...` : 打印一个或多个图形中的块和切割顶点的数量
- `bcomps -x -o 路径/到/输出.gv 路径/到/输入1.gv 路径/到/输入2.gv ...` : 根据 `输出.gv` 将每个块和块切割顶点树写入多个有编号的文件中
- `bcomps -?` : 显示帮助

## Links
- More information: https://graphviz.org/pdf/bcomps.1.pdf
