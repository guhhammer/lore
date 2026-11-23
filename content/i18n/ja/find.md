# find

ディレクトリツリー下のファイルやディレクトリを再帰的に検索します。

## Usage
- `find ルートパス -name '*.ext'` : 拡張子でファイル検索する
- `find ルートパス -path '*/path/*/*.ext' -or -name '*pattern*'` : 複数のパス/名前パターンに一致するファイルを検索する
- `find ルートパス -type d -iname '*lib*'` : 大文字小文字を区別しないモードで、指定された名前にマッチするディレクトリを検索する
- `find ルートパス -name '*.py' -not -path '*/site-packages/*'` : 特定のパスを除いて、与えられたパターンにマッチするファイルを検索する
- `find ルートパス -maxdepth 1 -size +500k -size -10M` : 与えられたサイズ範囲にマッチするファイルを検索し、再帰的な深さを"1"に制限する
- `find ルートパス -name '*.ext' -exec wc -l {} \;` : 各ファイルに対してコマンドを実行する (ファイル名にアクセスするにはコマンド内で`{}`を使用)
- `find ルートパス -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : 今日変更されたすべてのファイルを検索し、その結果を引数として1つのコマンドに渡す
- `find ルートパス -type f|d -empty -delete -print` : 空のファイルまたはディレクトリを検索し、冗長に削除する

## Links
- More information: https://manned.org/find
