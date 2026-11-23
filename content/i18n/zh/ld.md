# ld

将目标文件链接在一起。

## Usage
- `ld 路径/到/文件.o -o 路径/到/输出_可执行文件` : 将一个没有依赖项的特定目标文件链接到可执行文件中
- `ld 路径/到/文件1.o 路径/到/文件2.o -o 路径/到/输出_可执行文件` : 将两个目标文件链接在一起
- `ld -o 路径/到/输出_可执行文件 -I /lib/ld-linux-x86-64.so.2 /lib/crt1.o /lib/crti.o -lc 路径/到/文件.o /lib/crtn.o` : 将一个 x86_64 程序动态链接到 glibc (文件路径根据系统而变化)

## Links
- More information: https://sourceware.org/binutils/docs/ld.html#Options
