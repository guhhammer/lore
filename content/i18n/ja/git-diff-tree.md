# git diff-tree

2つのツリーオブジェクトから見つかったblobの内容とモードを比較する。

## Usage
- `git diff-tree ツリー指定1 ツリー指定2` : 2つのツリーオブジェクトを比較する
- `git diff-tree -r コミット1 コミット2` : 2つの指定したコミット間の変更を表示する
- `git diff-tree -p ツリー指定1 ツリー指定2` : パッチ形式で変更を表示する
- `git diff-tree ツリー指定1 ツリー指定2 -- ディレクトリ/サブディレクトリ/ファイル_または_ディレクトリ` : 指定したパスで変更を絞り込む

## Links
- More information: https://git-scm.com/docs/git-diff-tree
