# uv add

将包依赖添加到 `pyproject.toml` 文件。

## Usage
- `uv add 包名` : 添加一个包（最新版本）
- `uv add 包1 包2 ...` : 添加多个包
- `uv add 包名>=1.2.3` : 添加一个包，并限制其版本
- `uv add --optional 可选依赖组 包1 包2 ...` : 将包添加到可选依赖组，并在发布时包含在内
- `uv add --group 本地依赖组 package1 package2 ...` : 将包添加到本地依赖组，发布时不会包含在内
- `uv add --dev 包1 包2 ...` : 将包添加到 dev 本地依赖组，作为 `--group dev` 的简写
- `uv add --editable 路径/到/包/` : 添加包，并设定为可编辑的
- `uv add 包名 --extra 附加_功能` : 安装包时，启用一个附加功能（可以多次提供该参数）

## Links
- More information: https://peps.python.org/pep-0508/
