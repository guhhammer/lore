# uv sync

更新项目环境，来匹配锁文件 (lockfile)。

## Usage
- `uv sync` : 将项目环境与锁文件同步
- `uv sync --all-extras` : 同步，并且包含所有的可选依赖
- `uv sync --extra 可选依赖_名称` : 同步，包含特定的可选依赖
- `uv sync --only-dev` : 仅同步开发环境依赖
- `uv sync --no-dev` : 同步，但排除开发环境依赖
- `uv sync --group 依赖组_名称` : 同步特定的依赖组
- `uv sync --check` : 检查环境是否已经同步（不会做出更改）
- `uv sync --dry-run` : 预览同步会做出的更改，但是不执行

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-sync
