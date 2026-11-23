# umask

管理用户为新创建文件设置的读/写/执行权限掩码（即被限制的权限）。

## Usage
- `umask` : 显示当前掩码（八进制表示）
- `umask -S` : 以符号方式（人类可读格式）显示当前掩码
- `umask a+r` : 符号化更改掩码以允许所有用户的读取权限（掩码的其他部分保持不变）
- `umask 077` : 设置掩码（使用八进制）以对文件的所有者不限制任何权限，对其他所有人限制所有权限

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-umask
