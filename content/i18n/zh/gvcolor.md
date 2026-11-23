# gvcolor

用一系列颜色为有序有向图着色。

## Usage
- `gvcolor 路径/到/布局1.gv 路径/到/布局2.gv ... >路径/到/输出.gv` : 为一个或多个已被 `dot` 处理的有序有向图着色
- `dot 路径/到/输入.gv | gvcolor | dot -T png >路径/到/输出.png` : 对一个图进行布局和着色，然后将其转换为 PNG 图像
- `gvcolor -?` : 显示帮助

## Links
- More information: https://graphviz.org/pdf/gvcolor.1.pdf
