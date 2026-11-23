# msbuild

Microsoft の Visual Studio プロジェクトソリューション用ビルドツールです。

## Usage
- `msbuild` : 現在のディレクトリにある最初のプロジェクトファイルをビルドする
- `msbuild path/to/project_file` : 特定のプロジェクトファイルをビルドする
- `msbuild path/to/project_file /target:ターゲット名` : ビルドするターゲットを 1 つ以上指定する (セミコロン(;)で区切る)
- `msbuild path/to/project_file /property:プロパティ名=値` : プロパティを 1 つ以上指定する (セミコロン(;)で区切る)
- `msbuild path/to/project_file /toolsversion:バージョン` : 使用するビルドツールのバージョンを指定する
- `msbuild path/to/project_file /detailedsummary` : プロジェクトの設定方法に関する詳細情報をログの最後に表示する
- `msbuild /help` : ヘルプを表示する

## Links
- More information: https://learn.microsoft.com/visualstudio/msbuild
