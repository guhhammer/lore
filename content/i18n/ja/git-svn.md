# git svn

SubversionリポジトリとGitの間で双方向に操作する。

## Usage
- `git svn clone https://example.com/subversion_repo ローカルディレクトリ` : SVNリポジトリをクローンする
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo ローカルディレクトリ` : 指定したリビジョン番号からSVNリポジトリをクローンする
- `git svn rebase` : リモートSVNリポジトリからローカルクローンを更新する
- `git svn fetch` : gitの `HEAD` を変更せずにリモートSVNリポジトリから更新を取得する
- `git svn commit` : SVNリポジトリへコミットを戻す

## Links
- More information: https://git-scm.com/docs/git-svn
