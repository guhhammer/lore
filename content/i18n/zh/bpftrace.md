# bpftrace

Linux eBPF 的高级跟踪语言。

## Usage
- `sudo bpftrace -l` : 列出所有可用的探针
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'` : 运行单行程序（例如按程序进行系统调用计数）
- `sudo bpftrace 文件` : 从文件运行程序
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter /pid == 123/ { @[comm] = count(); }'` : 通过 PID 跟踪程序
- `sudo bpftrace -d -e '单行程序'` : 进行试运行并以 eBPF 格式显示输出
- `bpftrace -V` : 显示版本

## Links
- More information: https://github.com/bpftrace/bpftrace/blob/master/man/adoc/bpftrace.adoc
