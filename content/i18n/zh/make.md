# make

Makefile 文件描述目标的任务运行器。 通常用于控制源代码中可执行文件的编译。

## Usage
- `make` : 调用 Makefile 中指定的第一个目标（通常命名为 "all"）
- `make 目标` : 调用指定目标
- `make -j 4 目标` : 调用一个指定的目标，一次并行执行 4 个作业
- `make -f 文件` : 使用指定的 Makefile 文件
- `make -C 文件夹` : 从另一个目录执行 make
- `make -B 目标` : 即使源文件未更改，也强制执行目标
- `make 目标 变量=新值` : 覆盖在 Makefile 中定义的环境变量
- `make -e 目标` : 通过环境变量覆盖 Makefile 中定义的变量

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
