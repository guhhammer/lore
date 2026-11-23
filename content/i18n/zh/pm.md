# pm

安卓软件包管理工具。

## Usage
- `pm list packages` : 列出已安装的包
- `pm install /路径/到/软件包.apk` : 从指定路径，安装软件包
- `pm uninstall 软件包` : 从设备卸载一个包
- `pm clear 软件包` : 清除单个软件包的所有数据文件
- `pm enable 包名_或_类名` : 启用软件包或组件
- `pm disable-user 包名_或_类名` : 禁用软件包或组件
- `pm grant 包名 android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : 为应用授予权限
- `pm revoke 包名 android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : 撤销应用权限

## Links
- More information: https://developer.android.com/tools/adb#pm
