# sips

苹果的处理文件脚本系统。 光栅 / 查询图像 和 颜色同步 ICC 配置文件。

## Usage
- `sips --out 目标 / 文件夹 / 输出文件夹` : S 指定一个输出目录，这样原始文件就不会被修改
- `sips --resampleHeightWidth 1920 300 图片文件。扩展名` : 以指定的大小对图像重新采样，图像纵横比可能会更改
- `sips --resampleHeightWidthMax 1920 300 图片文件。扩展名` : 对图像重新取样，使高度和宽度不大于指定的大小（注意大写 Z）
- `sips --resampleWidth 960 目标 / 文件夹 / 所有图片文件` : 对目录中的所有图像重新取样，以适应 960 px 的宽度（保持纵横比）
- `sips --matchTo "/System/Library/ColorSync/Profiles/Generic RGB Profile.icc" 目标 / 文件夹 / 图片。扩展 --out 目标 / 文件夹 / 输出文件夹` : 将图像从 CMYK 转换为 RGB
- `sips --deleteProperty profile --deleteColorManagementProperties 目标 / 文件夹 / 图片。扩展` : 从图像中删除 ColorSync ICC 配置

## Links
- More information: https://keith.github.io/xcode-man-pages/sips.1.html
