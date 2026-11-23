# read

从标准输入（或文件）读取一行并将单词分配给变量。

## Usage
- `read 变量` : 读取键盘输入的数据赋值给变量
- `read -a 数组` : 将您输入的每一行存储为数组一个元素
- `read -n 字符数 变量` : 指定要读取的字符数，将数据赋值给变量
- `read _ 变量1 _ 变量2 <<<"The surname is Bond"` : 将多个数据依次赋值给多个变量
- `read -r 变量` : 读取键盘输入的数据赋值给变量，不对`\`进行转义
- `read -p "提示语：" 变量` : 键盘输入前显示提示语
- `read -s 变量` : 静默模式（如果输入来自终端，则不回显字符）读取键盘输入的数据赋值给变量
- `while read line; do echo|ls|rm|... "$line"; done <标准输入|路径/到/文件|...` : 从标准输入读取每一行进行操作

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-read
