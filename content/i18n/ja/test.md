# test

条件を評価します。 条件が真と評価された場合は 0 を、偽と評価された場合は 1 を返します。

## Usage
- `test "$変数名" = "/bin/zsh"` : 与えられた変数が与えられた文字列と等しいかどうかをテスト
- `test -z "$変数名"` : 与えられた変数が空であるかどうかをテスト
- `test -f "ファイルへのパス"` : ファイルが存在するかどうかをテスト
- `test ! -d "ディレクトリへのパス"` : ディレクトリが存在しないかどうかをテスト
- `test 条件 && echo "真" || echo "偽"` : if-else 文

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/test-invocation.html
