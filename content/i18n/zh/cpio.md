# cpio

将文件复制到归档中或从归档中复制出来。 支持以下归档格式：cpio 自定义二进制格式、旧 ASCII、新 ASCII、crc、HPUX 二进制格式、HPUX 旧 ASCII、旧 tar 以及 POSIX.1 tar。

## Usage
- `echo "路径/到/文件1 路径/到/文件2 ..." | cpio -o >目标文件.cpio` : 从标准输入读取文件名列表，并以 cpio 二进制格式将文件写入归档（copy-out 模式）
- `find 路径/到/目录 | cpio -ov >目标文件.cpio` : 递归复制目录中的所有文件和子目录并写入归档（copy-out 模式），同时显示详细输出
- `cpio <源文件.cpio -idv` : 从归档中提取所有文件（copy-in 模式），按需创建目录，同时显示详细输出

## Links
- More information: https://www.gnu.org/software/cpio/manual/cpio.html#Invoking-cpio
