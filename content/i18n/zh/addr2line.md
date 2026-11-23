# addr2line

将二进制文件地址转换成文件名和行数。

## Usage
- `addr2line -e 可执行文件路径 地址` : 显示可执行文件的指令地址对应源代码的文件名和行数
- `addr2line -e 可执行文件路径 -f 地址` : 显示函数名、文件名和行数
- `addr2line -e 可执行文件地址 -f -C 地址` : 将 C++ 代码函数名符号重组

## Links
- More information: https://manned.org/addr2line
