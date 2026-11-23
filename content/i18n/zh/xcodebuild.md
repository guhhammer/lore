# xcodebuild

构建 Xcode 项目。

## Usage
- `xcodebuild -workspace 工作区名.工作区 -scheme 主题名 -configuration 配置名 clean build SYMROOT=SYMROOT_路径` : 构建工作区
- `xcodebuild -target 目标名 -configuration 配置名 clean build SYMROOT=SYMROOT_路径` : 构建项目
- `xcodebuild -showsdks` : 显示 SDK

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
