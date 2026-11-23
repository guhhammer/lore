# git submodule

サブモジュールを確認・更新・管理する。

## Usage
- `git submodule` : 既存のサブモジュールと各サブモジュールでチェックアウトされているコミットを表示する
- `git submodule update --init --recursive` : リポジトリのサブモジュール (`.gitmodules` に記載されたもの) をインストールする
- `git submodule add リポジトリURL` : gitリポジトリを現在のリポジトリのサブモジュールとして追加する
- `git submodule add リポジトリURL ディレクトリ/サブディレクトリ` : gitリポジトリを指定したディレクトリに現在のリポジトリのサブモジュールとして追加する
- `git submodule update --remote` : サブモジュールを最新のコミットに更新する
- `git submodule set-url ディレクトリ/サブディレクトリ/サブモジュール 新しいURL` : サブモジュールのURLを変更する
- `git submodule deinit ディレクトリ/サブディレクトリ/サブモジュール` : サブモジュールの登録を解除する (例: `git rm` でリポジトリから削除する前)

## Links
- More information: https://git-scm.com/docs/git-submodule
