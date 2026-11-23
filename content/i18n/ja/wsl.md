# wsl

Windows Subsystem for Linux を管理します。

## Usage
- `wsl シェルコマンド` : linuxシェルを起動する(デフォルトのディストリビューションの場合)
- `wsl -e コマンド コマンド引数` : シェルを使わずにLinuxコマンドを実行する
- `wsl -d ディストリビューション シェルコマンド` : 特定のディストリビューションを指定する
- `wsl -l` : 利用可能なディストリビューションを一覧表示する
- `wsl --export ディストリビューション ディストロファイルパス.tar` : ディストリビューションを`.tar`ファイルにエクスポートする
- `wsl --import ディストリビューション インストール先パス ディストロファイルパス.tar` : ディストリビューションを`.tar`ファイルからインポートする
- `wsl --set-version ディストリビューション バージョン` : 指定したディストリビューションで使用するwslのバージョンを変更する
- `wsl --shutdown` : windows Subsystem for Linux をシャットダウンする

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
