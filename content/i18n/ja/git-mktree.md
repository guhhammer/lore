# git mktree

`ls-tree` 形式のテキストを使ってツリーオブジェクトを構築する。

## Usage
- `git mktree` : ツリーオブジェクトを構築し、各ツリーエントリのハッシュが既存オブジェクトを指していることを検証する
- `git mktree --missing` : 欠落したオブジェクトを許可する
- `git mktree -z` : ツリーオブジェクトのNUL終端出力 (`git ls-tree -z`) を読み取る
- `git mktree --batch` : 複数のツリーオブジェクトの作成を許可する
- `git <ディレクトリ/サブディレクトリ/tree.txt mktree` : `stdin` から読み取った内容を並べ替えてツリーを構築する (非再帰の `git ls-tree` 出力形式が必要)

## Links
- More information: https://git-scm.com/docs/git-mktree
