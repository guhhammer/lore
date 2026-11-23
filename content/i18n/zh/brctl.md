# brctl

以太网桥管理。

## Usage
- `sudo brctl show` : 显示有关当前现有以太网网桥信息的列表
- `sudo brctl add 网桥名` : 创建新的以太网桥接接口
- `sudo brctl del 网桥名` : 删除一个已存在的以太网桥接接口
- `sudo brctl addif 网桥名 接口名` : 向现有网桥添加接口
- `sudo brctl delif 网桥名 接口名` : 从现有网桥中删除接口

## Links
- More information: https://manned.org/brctl
