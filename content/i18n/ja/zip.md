# zip

ファイルを Zip アーカイブにパッケージして圧縮(アーカイブ)します。

## Usage
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : 指定したアーカイブに、ファイル/ディレクトリを追加する (recursively)
- `zip -d path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : 指定したアーカイブから、ファイル/ディレクトリを削除する (delete)
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ... -x path/to/excluded_files_or_directories` : 指定されたファイル/ディレクトリ以外 (excluding) をアーカイブする
- `zip -r -0..9 path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : ファイルやディレクトリを、指定の圧縮レベルでアーカイブする (`0` - 最低、 `9` - 最高)
- `zip -re path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : パスワードで暗号化 (encrypted) されたアーカイブを作成
- `zip -rs 3g path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : ファイル/ディレクトリを複数に分割 (split) された Zip アーカイブ(例えば 3GB の部分)にアーカイブする
- `zip -sf path/to/compressed.zip` : 指定したアーカイブの内容を表示する

## Links
- More information: https://manned.org/zip
