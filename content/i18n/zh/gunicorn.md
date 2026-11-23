# gunicorn

Python 的 WSGI HTTP 服务器。

## Usage
- `gunicorn 导入路径：应用程序` : 运行 Python web 应用程序
- `gunicorn -b localhost:8080 导入路径：应用程序` : 在 localhost 上监听 8080 端口
- `gunicorn --reload 导入路径：应用程序` : 启用实时自动加载
- `gunicorn -w 4 导入路径：应用程序` : 使用 4 个工作进程处理请求
- `gunicorn --threads 4 导入路径：应用程序` : 使用 4 个工作线程处理请求
- `gunicorn --certfile cert.pem --keyfile key.pem 导入路径：应用程序` : 通过 HTTPS 运行应用程序

## Links
- More information: https://manned.org/gunicorn
