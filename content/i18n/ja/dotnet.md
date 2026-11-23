# dotnet

.NET Core 用のクロスプラットフォーム .NET コマンドラインツール。 `build` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `dotnet new テンプレート短縮名` : .NET プロジェクトを初期化する
- `dotnet restore` : nuGet パッケージを復元する
- `dotnet run` : 現在のディレクトリで .NET プロジェクトをビルドして実行する
- `dotnet path/to/application.dll` : パッケージ化された dotnet アプリケーションを実行する (必要なのはランタイムだけで、残りのコマンドには.NET Core SDK がインストールされている必要がある)

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools
