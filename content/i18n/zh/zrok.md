# zrok

将本地服务和文件暴露到互联网。 OpenZiti 项目的一部分，提供安全的零信任共享。

## Usage
- `zrok invite` : 请求使用公共 zrok 服务的邀请（首先运行此命令）
- `zrok enable 你的令牌` : 使用邀请邮件中的令牌启用 zrok 环境
- `zrok share public http://localhost:8080` : 为本地 Web 服务器创建一个可公开访问的 URL
- `zrok share private http://localhost:3000` : 创建一个仅可通过唯一令牌访问的安全共享
- `zrok access private 共享令牌` : 访问由其他用户创建的私有共享
- `zrok share public --backend-mode web 路径/到/目录` : 将本地目录的内容作为一个简单的网站提供服务
- `zrok status` : 显示 zrok 环境的状态以及当前活动的共享

## Links
- More information: https://docs.zrok.io/
