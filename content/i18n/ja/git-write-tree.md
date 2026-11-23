# git write-tree

現在のインデックスからツリーオブジェクトを作成する低レベルユーティリティ。

## Usage
- `git write-tree` : 現在のインデックスからツリーオブジェクトを作成する
- `git write-tree --missing-ok` : ディレクトリから参照されるオブジェクトがオブジェクトデータベースに存在するか確認せずに、ツリーオブジェクトを作成する
- `git write-tree --prefix サブディレクトリ/` : サブディレクトリを表すツリーオブジェクトを作成する (指定したサブディレクトリ内のサブプロジェクト用ツリーオブジェクトを書き込むために使用)

## Links
- More information: https://git-scm.com/docs/git-write-tree
