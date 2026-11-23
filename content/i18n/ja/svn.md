# svn

Subversion のコマンドラインクライアントツールです。

## Usage
- `svn co url/to/repository` : リポジトリから作業コピーをチェックアウトする
- `svn up` : リポジトリからの変更を作業コピーに取り込む
- `svn add パス/宛先/ファイル_または_ディレクトリ` : ファイルやディレクトリをバージョン管理下に置き、リポジトリへの追加をスケジューリングする。これらは次のコミットで追加される
- `svn ci -m コミットログメッセージ パス/宛先/ディレクトリ` : 作業コピーの変更をリポジトリに送信する
- `svn log -vl 10` : 過去10リビジョンの変更点を表示し、各リビジョンで変更されたファイルを表示する
- `svn h` : ヘルプを表示する

## Links
- More information: https://svnbook.red-bean.com/en/1.7/svn-book.html#svn.ref.svn
