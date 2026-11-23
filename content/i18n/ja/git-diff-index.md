# git diff-index

作業ディレクトリをコミットまたはツリーオブジェクトと比較する。

## Usage
- `git diff-index コミット` : 作業ディレクトリを指定したコミットと比較する
- `git diff-index コミット ディレクトリ/サブディレクトリ/ファイル_または_ディレクトリ` : 作業ディレクトリ内の指定したファイルまたはディレクトリをコミットと比較する
- `git diff-index --cached コミット` : ステージされた変更を確認するため、作業ディレクトリをインデックス (ステージングエリア) と比較する
- `git diff-index --quiet コミット` : 出力を抑制し、差分があるかを終了ステータスで返す

## Links
- More information: https://git-scm.com/docs/git-diff-index
