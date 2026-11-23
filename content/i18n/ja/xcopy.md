# xcopy

ファイルとディレクトリツリーをコピーします。

## Usage
- `xcopy path\to\file_or_directory path\to\destination_directory` : ファイル(単独または複数)を指定された宛先にコピーする
- `xcopy path\to\file_or_directory path\to\destination_directory /p` : コピーの前に、コピーするファイルを一覧表示する
- `xcopy path\to\file_or_directory path\to\destination_directory /t` : ファイルを除いてディレクトリ構造だけをコピーする
- `xcopy path\to\file_or_directory path\to\destination_directory /e` : コピー時に、空のディレクトリを含める
- `xcopy path\to\file_or_directory path\to\destination_directory /o` : コピー元のACLを、コピー先に残す
- `xcopy path\to\file_or_directory path\to\destination_directory /z` : ネットワーク接続が切れたときに、再開できるようにする
- `xcopy path\to\file_or_directory path\to\destination_directory /y` : コピー先にファイルが存在する場合、プロンプトを表示しない
- `xcopy /?` : ヘルプを表示する
