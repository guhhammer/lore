# file

确定文件类型。

## Usage
- `file 文件名` : 提供指定文件类型的描述，对于没有文件扩展名的文件可以正常工作
- `file -z xxx.zip` : 查看压缩文件并确定其中的文件类型
- `file -s 文件名` : 允许文件与特殊文件或设备文件一起使用
- `file -k 文件名` : 不要在第一个文件类型匹配时停止；继续执行直到文件结束
- `file -i 文件名` : 确定文件的 MIME 编码类型

## Links
- More information: https://manned.org/file
