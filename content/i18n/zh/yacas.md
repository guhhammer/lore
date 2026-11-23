# yacas

另一个计算机代数系统。

## Usage
- `yacas` : 启动交互式的 `yacas` 会话
- `Integrate(x)Cos(x);` : [交互式] 在 `yacas` 会话中，执行一个语句
- `Example();` : [交互式] 在 `yacas` 会话中，显示一个示例
- `quit` : [交互式] 从 `yacas` 会话中退出
- `yacas -p -c 路径/到/脚本1 路径/到/脚本2 ...` : 执行一个或多个 `yacas` 脚本（没有终端或提示符），然后退出
- `echo "Echo( Deriv(x)Cos(1/x) );" | yacas -p -c /dev/stdin` : 执行并打印一个语句的结果，然后退出

## Links
- More information: https://www.yacas.org/
