# chmod

ファイルやディレクトリのアクセス権限を変更します。

## Usage
- `chmod u+x path/to/file` : ファイルを所有する user に、そのファイルを実行 (xecute) する権限を付与する
- `chmod u+rw path/to/file_or_directory` : user に、ファイル/ディレクトリの 読み取り (read) と 書き込み (write) の権限を付与する
- `chmod g-x path/to/file` : group から 実行 (executable) 権限を削除する
- `chmod a+rx path/to/file` : 全て (all) のユーザに read と execute の権限を付与する
- `chmod o=g path/to/file` : (ファイル所有者のグループに属していない) その他のユーザ (others) に group と同じ権限を付与する
- `chmod o= path/to/file` : その他のユーザ (others) から全ての権限を削除する
- `chmod -R g+w,o+w path/to/directory` : group と others に write 権限を再帰的に付与する
- `chmod -R a+rX path/to/directory` : 全て (all) のユーザーに、ファイルへの read 権限と、ディレクトリ内のサブディレクトリへの eXecute 権限を再帰的に付与する
