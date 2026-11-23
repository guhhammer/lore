# heroku

从命令行创建和管理 Heroku 应用。

## Usage
- `heroku login` : 登录到你的 heroku 帐户
- `heroku create` : 创建一个 heroku 应用
- `heroku logs --app app_name` : 显示应用的日志
- `heroku run process_name --app app_name` : 在 dyno（Heroku 虚拟机）中运行一次性进程
- `heroku ps --app app_name` : 列出应用的 dyno（Heroku 虚拟机）
- `heroku destroy --app app_name` : 永久销毁应用
