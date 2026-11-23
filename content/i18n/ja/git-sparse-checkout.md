# git sparse-checkout

すべてをクローンまたはチェックアウトする代わりに、リポジトリ内の一部のファイルだけをチェックアウトする。

## Usage
- `git sparse-checkout init` : スパースチェックアウトを有効にする
- `git sparse-checkout disable` : スパースチェックアウトを無効にし、リポジトリ全体を復元する
- `git sparse-checkout set ディレクトリ/サブディレクトリ` : 含めるディレクトリ (またはファイル) を指定する
- `git sparse-checkout add ディレクトリ/サブディレクトリ` : 後からパスを追加する

## Links
- More information: https://manned.org/git-sparse-checkout
