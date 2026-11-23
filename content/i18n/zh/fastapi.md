# fastapi

运行 FastAPI 应用的命令行工具，底层使用 Uvicorn。

## Usage
- `fastapi run 路径/文件.py --reload` : 使用自动重载运行 FastAPI 应用（用于开发）
- `fastapi dev 路径/文件.py` : 在开发模式下运行应用
- `fastapi run 路径/文件.py --host 主机地址 --port 端口` : 指定运行的主机和端口
- `fastapi run 路径/文件.py --app-dir 路径/到/应用 --app 自定义_app_名称` : 设置应用变量名（如果不是 `app`）或指定自定义应用目录
- `fastapi --help` : 显示帮助
- `fastapi 子命令 --help` : 显示子命令的帮助信息

## Links
- More information: https://manned.org/fastapi
