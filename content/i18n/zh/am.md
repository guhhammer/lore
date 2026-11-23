# am

Android 活动管理器。

## Usage
- `am start -n com.android.settings/.Settings` : 启动一个指定的活动
- `am start -a android.intent.action.VIEW -d tel:123` : 启动一个活动并将数据传递给它
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : 启动与特定操作和类别匹配的活动
- `am to-uri -a android.intent.action.VIEW -d tel:123` : 将意图转换为 URI

## Links
- More information: https://developer.android.com/tools/adb#am
