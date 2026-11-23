# adb

安卓调试桥：与 Android 模拟器或已连接的 Android 设备通信。

## Usage
- `adb start-server` : 检查 adb server 进程的是否在运行，并开启它
- `adb kill-server` : 终止 adb server 进程
- `adb shell` : 在目标模拟器 / 设备实例上开启一个远程 shell
- `adb install -r 路径/到/应用.apk` : 将 Android 应用程序推送到模拟器 / 设备
- `adb pull 路径/到/设备的文件或目录 路径/到/本地目录` : 从目标设备上拷贝一个文件 / 目录到本地
- `adb push 路径/到/本地文件或目录 路径/到/设备目录` : 从本地拷贝一个文件 / 目录到目标设备
- `adb devices` : 列出已连接的设备
- `adb -s 设备_id shell` : 当有多个设备连接时，指定目标设备执行命令

## Links
- More information: https://developer.android.com/tools/adb
