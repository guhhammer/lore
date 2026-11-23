# git maintenance

Gitリポジトリデータを最適化するタスクを実行する。

## Usage
- `git maintenance register` : 現在のリポジトリを、毎日メンテナンスが実行されるユーザーのリポジトリ一覧に登録する
- `git maintenance start` : 現在のリポジトリでメンテナンスタスクが毎時実行されるようにスケジュールする
- `git maintenance stop` : 現在のリポジトリのバックグラウンドメンテナンススケジュールを停止する
- `git maintenance unregister` : 現在のリポジトリをユーザーのメンテナンス対象リポジトリ一覧から削除する
- `git maintenance run --task commit-graph|gc|incremental-repack|loose-objects|pack-refs|prefetch` : 現在のリポジトリで指定したメンテナンスタスクを実行する

## Links
- More information: https://git-scm.com/docs/git-maintenance
