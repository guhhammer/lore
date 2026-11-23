# robocopy

堅牢なファイルとフォルダのコピー。 デフォルトでは、コピー元とコピー先でタイムスタンプが異なるか、ファイルサイズが異なる場合のみ、ファイルがコピーされます。

## Usage
- `robocopy path\to\source_directory path\to\destination_directory *.jpg *.bmp` : すべての `.jpg` と `.bmp` ファイルをあるディレクトリから、別のディレクトリにコピーする
- `robocopy path\to\source_directory path\to\destination_directory /E` : 空のファイルも含めて、すべてのファイルとサブディレクトリをコピーする
- `robocopy path\to\source_directory path\to\destination_directory /MIR /COPYALL` : ディレクトリをミラー/同期し、ソースにないものを削除し、すべての属性とパーミッションを含める
- `robocopy path\to\source_directory path\to\destination_directory /E /XO` : コピー先のファイルより古いソースファイルを除いて、すべてのファイルとサブディレクトリをコピーする
- `robocopy path\to\source_directory path\to\destination_directory /MIN:52428800 /L` : 50MB以上のファイルをコピーする代わりに一覧表示する
- `robocopy path\to\source_directory path\to\destination_directory /Z /R:5 /W:15` : ネットワーク接続が失われた場合の再開を許可し、再試行を5回、待機時間を15秒に制限する
- `robocopy /?` : ヘルプを表示する
