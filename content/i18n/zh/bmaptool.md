# bmaptool

便捷地创建或复制块文件映射（被设计的比`cp`或`dd`更快）。

## Usage
- `bmaptool create -o blockmap 格式文件.bmap 图片文件` : 使用图片生成块图文件
- `bmaptool copy --bmap blockmap 格式文件 图片文件 /开发路径/sdb` : 复制图片到指定目录
- `bmaptool copy --bmap blockmap 格式文件 图片文件.gz /开发路径/sdb` : 复制压缩后的图片到指定目录
- `bmaptool copy --nobmap 图片文件 /开发路径/sdb` : 复制图片的时候，不将图片转成块图

## Links
- More information: https://manned.org/bmaptool
