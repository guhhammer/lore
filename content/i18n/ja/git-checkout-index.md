# git checkout-index

インデックスから作業ツリーへファイルをコピーする。

## Usage
- `git checkout-index -a` : 最後のコミット以降に削除されたファイルを復元する
- `git checkout-index -a -f` : 最後のコミット以降に削除または変更されたファイルを復元する
- `git checkout-index -a -f -n` : 最後のコミット以降に変更されたファイルを復元し、削除されたファイルは無視する
- `git checkout-index -a -f --prefix ディレクトリ/サブディレクトリ/エクスポート先ディレクトリ/` : 最後のコミット時点のツリー全体のコピーを指定したディレクトリへエクスポートする (末尾のスラッシュが重要)

## Links
- More information: https://git-scm.com/docs/git-checkout-index
