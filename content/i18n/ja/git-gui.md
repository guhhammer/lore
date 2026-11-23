# git gui

ブランチ、コミット、リモートを管理し、ローカルマージを実行するGit用GUI。

## Usage
- `git gui` : GUIを起動する
- `git gui blame ディレクトリ/サブディレクトリ/ファイル` : 各行に作成者名とコミットハッシュを付けて指定したファイルを表示する
- `git gui blame リビジョン ディレクトリ/サブディレクトリ/ファイル` : 指定したリビジョンで `git gui blame` を開く
- `git gui blame --line=行番号 ディレクトリ/サブディレクトリ/ファイル` : `git gui blame` を開き、指定した行が中央に来るよう表示位置を移動する
- `git gui citool` : 1つのコミットを作成するためのウィンドウを開き、完了後にシェルへ戻る
- `git gui citool --amend` : 「直前のコミットを修正」モードで `git gui citool` を開く
- `git gui citool --nocommit` : 読み取り専用モードで `git gui citool` を開く
- `git gui browser maint` : 指定したブランチのツリーを閲覧し、ファイルをクリックするとblameツールを開く

## Links
- More information: https://git-scm.com/docs/git-gui
