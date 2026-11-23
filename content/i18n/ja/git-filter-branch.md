# git filter-branch

ファイル削除など、ブランチ履歴を変更する。

## Usage
- `git filter-branch --tree-filter 'rm -f ファイル' HEAD` : すべてのコミットからファイルを削除する
- `git filter-branch --env-filter 'GIT_AUTHOR_EMAIL=新しいメールアドレス' HEAD` : 作成者のメールアドレスを更新する
- `git filter-branch --tree-filter 'rm -rf フォルダー' HEAD` : 履歴からフォルダーを削除する

## Links
- More information: https://git-scm.com/docs/git-filter-branch
