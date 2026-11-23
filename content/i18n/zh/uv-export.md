# uv export

将项目的锁文件 (lockfile) 导出为其它格式。

## Usage
- `uv export --format requirements-txt -o requirements.txt` : 将依赖项导出到 `requirements.txt` 文件
- `uv export --format pylock.toml` : 将依赖项导出为 `pylock.toml` 格式
- `uv export --no-dev` : 仅导出生产环境的依赖项（不包括开发环境依赖）
- `uv export --extra 依赖组_名称` : 仅导出一个特定的依赖组
- `uv export --all-extras` : 导出，并包含所有可选依赖
- `uv export --group 依赖组_名称` : 导出，并包含特定的依赖组
- `uv export --no-hashes` : 导出，但不包含哈希值
- `uv export --package 程序_包名` : 为工作区的特定程序包，导出依赖项

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-export
