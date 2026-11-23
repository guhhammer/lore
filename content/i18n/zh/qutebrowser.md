# qutebrowser

一个基于 PyQt5 的键盘驱动、类似 vim 的浏览器。

## Usage
- `qutebrowser --basedir 路径/到/目录` : 使用指定存储目录打开 qutebrowser
- `qutebrowser --set content.geolocation true|false` : 使用临时设置打开 qutebrowser 实例
- `qutebrowser --restore 会话名称` : 恢复一个 qutebrowser 实例的指定会话
- `qutebrowser --target auto|tab|tab-bg|tab-silent|tab-bg-silent|window|private-window` : 启动 qutebrowser，使用指定方式打开所有 URL
- `qutebrowser --temp-basedir --json-logging` : 使用临时基础目录打开 qutebrowser，并以 JSON 格式将日志打印到 `stdout`

## Links
- More information: https://qutebrowser.org/doc/qutebrowser.1.html
