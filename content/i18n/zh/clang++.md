# clang++

编译 C++ 源文件。 LLVM 项目的组件之一。

## Usage
- `clang++ 源/文件/的路径1.cpp 源/文件/的路径2.cpp ... -o 可执行/文件/的路径` : 将一组源文件编译为二进制文件
- `clang++ 源/文件/的路径.cpp -Wall -o 可执行文件` : 打印所有错误和警告
- `clang++ 源/文件/的路径.cpp -Wall -g -Og -o 可执行/文件/的路径` : 打印普通警告和调试信息, 并在不影响调试的情况下优化
- `clang++ 源/文件/的路径.cpp -std=c++20 -o 可执行/文件/的路径` : 指定 C++ 语言标准
- `clang++ 源/文件/的路径.cpp -o 可执行/文件/的路径 -I头/文件/的目录 -L库/的目录 -l库的名字` : 包含与源文件不在同一路径下的库
- `clang++ -S -emit-llvm 源/文件/的路径.cpp -o IR/的/路径.ll` : 将源文件编译为 LLVM 中间表示（IR）
- `clang++ 源/文件/的路径.cpp -O1|2|3|fast -o 可执行/文件/的路径` : 优化编译后程序的性能
- `clang++ --version` : 显示版本

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
