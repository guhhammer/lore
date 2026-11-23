# gcc

预处理和编译 C 和 C++ 源文件，然后汇编并将他们链接起来。 GCC (GNU Compiler Collection) 项目的组件之一。

## Usage
- `gcc 源/文件/的路径1.cpp 源/文件/的路径2.cpp ... -o 可执行/文件/的路径` : 将一组源文件编译为二进制文件
- `gcc 源/文件/的路径.cpp -Wall -o output_executable` : 打印所有错误和警告
- `gcc 源/文件/的路径.cpp -Wall -g -Og -o 可执行/文件/的路径` : 打印普通警告和调试信息, 并在不影响调试的情况下优化
- `gcc 源/文件/的路径.c -o 可执行/文件/的路径 -I头/文件/的目录 -L库/的目录 -l库的名字` : 包含与源文件不在同一路径下的库
- `gcc -S 源/文件/的路径` : 将源文件编译为汇编指令
- `gcc -c 源/文件/的路径1.c 源/文件/的路径2.c...` : 将源文件编译成目标文件（.o）而不链接
- `gcc 源/文件/的路径.cpp -O1|2|3|fast -o 可执行/文件/的路径` : 优化编译后程序的性能
- `gcc --version` : 显示版本

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
