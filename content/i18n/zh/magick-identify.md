# magick identify

输出图片文件的格式与属性。

## Usage
- `magick identify 路径/到/图片` : 输出图片的格式以及一些基本属性
- `magick identify -verbose 路径/到/图片` : 输出图片的格式及其详细属性
- `magick identify -format "%f,%w,%h\n" *.jpg >路径/到/文件列表.csv` : 将当前目录下的所有 JPEG 图片的尺寸属性

## Links
- More information: https://imagemagick.org/script/identify.php
