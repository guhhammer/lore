# conda

为任何编程语言准备的包、依赖和环境管理工具。 部分子命令（例如 `create`）有自己的文档。

## Usage
- `conda create -n 环境_名称 python=3.9 matplotlib` : 创建新的环境，并将指定包安装入该环境
- `conda info -e` : 列出所有环境
- `conda activate 环境_名称` : 激活某个环境
- `conda deactivate` : 取消激活某个环境
- `conda remove -n 环境_名称 --all` : 删除一个环境（会移除所有的包）
- `conda install python=3.4 numpy` : 将多个包按照到当前环境
- `conda list` : 列出当前环境中的已安装包
- `conda clean -a` : 删除未使用的包和缓存

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
