# mingle

捆绑图形布局中的边缘。

## Usage
- `mingle 路径/到/布局1.gv 路径/到/布局2.gv ... >路径/到/输出.gv` : 捆绑一个或多个已经有布局信息的图形布局的边缘
- `dot 路径/到/输入.gv | mingle | dot -T png >路径/到/输出.png` : 通过一个命令执行布局、捆绑和输出到图片
- `mingle -?` : 显示帮助

## Links
- More information: https://www.graphviz.org/pdf/mingle.1.pdf
