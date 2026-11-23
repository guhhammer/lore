# modprobe

从 Linux 内核添加或移除模块。

## Usage
- `modprobe -n 模块名` : 假装将模块加载到内核中，但实际上不这样做
- `sudo modprobe 模块名` : 加载模块到内核
- `sudo modprobe -r 模块名` : 从内核移除模块
- `sudo modprobe -r --remove-holders 模块名` : 从内核移除模块及所有依赖它的模块
- `sudo modprobe -D 模块名` : 显示内核模块的依赖

## Links
- More information: https://manned.org/modprobe
