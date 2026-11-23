# grubby

用于配置 `grub` 和 `zipl` 引导加载器的工具。

## Usage
- `sudo grubby --update-kernel=ALL --args 'quiet console=ttyS0'` : 添加内核启动参数到所有内核菜单项
- `sudo grubby --update-kernel=DEFAULT --remove-args quiet` : 从默认内核的条目中移除现有参数
- `sudo grubby --info=ALL` : 列出所有内核菜单项

## Links
- More information: https://manned.org/grubby.8
