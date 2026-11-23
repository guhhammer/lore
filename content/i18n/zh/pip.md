# pip

Python 主流的包安装管理工具。

## Usage
- `pip install 包名` : 安装包（通过 `pip install` 查看更多安装示例）
- `pip install --user 包名` : 安装包到用户目录而不是系统范围的默认位置
- `pip install -U 包名` : 升级包
- `pip uninstall 包名` : 卸载包
- `pip freeze >requirements.txt` : 将已安装的包以 Requirements 的格式保存文件中
- `pip show 包名` : 查看包的详细信息
- `pip install -r requirements.txt` : 通过依赖文件（如 `requirements.txt`）来进行安装

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip/
