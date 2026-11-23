# chown

ファイルやディレクトリのユーザとグループの所有者を変更します。

## Usage
- `chown ユーザ名 path/to/file_or_directory` : ファイル/ディレクトリの所有ユーザを変更
- `chown ユーザ名:グループ名 path/to/file_or_directory` : ファイル/ディレクトリの所有ユーザとグループを変更
- `chown ユーザ名: path/to/file_or_directory` : ファイル/ディレクトリの所有ユーザとグループを `user` という名前に変更する
- `chown -R ユーザ名 path/to/directory` : ディレクトリとそのコンテンツの所有者を、再帰的に変更する
- `chown -h ユーザ名 path/to/symlink` : シンボリックリンクの所有者を変更
- `chown --reference path/to/reference_file path/to/file_or_directory` : 参照ファイルに合わせてファイル/ディレクトリの所有者を変更
