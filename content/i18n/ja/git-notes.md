# git notes

オブジェクトのノートを追加または確認する。

## Usage
- `git notes list` : すべてのノートと、それらが付けられているオブジェクトを一覧表示する
- `git notes list オブジェクト` : 指定したオブジェクトに付けられているすべてのノートを一覧表示する (既定は `HEAD`)
- `git notes show オブジェクト` : 指定したオブジェクトに付けられているノートを表示する (既定は `HEAD`)
- `git notes append オブジェクト` : 指定したオブジェクトにノートを追加する (既定のテキストエディターを開く)
- `git notes append --message="メッセージテキスト"` : メッセージを指定して、指定したオブジェクトにノートを追加する
- `git notes edit オブジェクト` : 既存のノートを編集する (既定は `HEAD`)
- `git notes copy コピー元オブジェクト コピー先オブジェクト` : あるオブジェクトから別のオブジェクトへノートをコピーする
- `git notes remove オブジェクト` : 指定したオブジェクトに追加されたすべてのノートを削除する

## Links
- More information: https://git-scm.com/docs/git-notes
