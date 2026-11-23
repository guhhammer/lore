# jpegoptim

优化 JPEG 图像。

## Usage
- `jpegoptim 图像1.jpeg 图像2.jpeg 图像3.jpeg ...` : 优化一组 JPEG 图像，保留所有相关数据
- `jpegoptim -s 图像1.jpeg 图像2.jpeg 图像3.jpeg ...` : 优化 JPEG 图像，剥离所有非必要数据
- `jpegoptim --all-progressive 图像1.jpeg 图像2.jpeg 图像3.jpeg ...` : 强制输出图像为渐进式
- `jpegoptim -S 250k 图像1.jpeg 图像2.jpeg 图像3.jpeg ...` : 强制输出图像具有固定的最大文件大小

## Links
- More information: https://manned.org/jpegoptim
