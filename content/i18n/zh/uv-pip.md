# uv pip

提供类似 pip 的命令，用于安装、卸载和管理程序包。

## Usage
- `uv pip install 包名` : 安装一个包
- `uv pip install -r requirements.txt` : 根据程序包依赖需求文件，安装程序包
- `uv pip install 包名==1.2.3` : 安装特定版本的一个包
- `uv pip uninstall 包名` : 卸载一个包
- `uv pip freeze >requirements.txt` : 将安装的包列表，保存到文件
- `uv pip list` : 列出安装的包
- `uv pip show 包名` : 展示关于某个已安装包的信息
- `uv pip sync requirements.txt` : 将环境与程序包依赖需求文件进行同步（将会安装/卸载程序包，来完全匹配该文件）

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-pip
