# git prune

到達不能なすべてのオブジェクトをオブジェクトデータベースから削除するGitコマンド。 このコマンドは直接使われることは少なく、多くの場合Git gcから使われる内部コマンドとして使用される。

## Usage
- `git prune -n` : git pruneで削除される内容を、実際には削除せずに報告する
- `git prune -v` : 到達不能なオブジェクトを削除し、削除された内容を `stdout` に表示する
- `git prune --progress` : 進捗を表示しながら到達不能なオブジェクトを削除する

## Links
- More information: https://git-scm.com/docs/git-prune
