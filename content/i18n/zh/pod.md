# pod

Swift 和 Objective-C Cocoa 项目的依赖关系管理。

## Usage
- `pod init` : 为当前项目初始化包含默认内容的 podfile
- `pod install` : 下载并安装 pod 文件中定义的所有 pod（以前未安装）
- `pod list` : 列出所有可用的 pod
- `pod outdated` : 显示过时的 pod（当前安装的 pod）
- `pod update` : 将当前安装的所有 pod 更新到其最新版本
- `pod update pod_名` : 将特定（以前安装的）pod 更新为其最新版本
- `pod deintegrate xcode_项目` : 从 Xcode 项目中删除 CocoaPods

## Links
- More information: https://guides.cocoapods.org/terminal/commands.html
