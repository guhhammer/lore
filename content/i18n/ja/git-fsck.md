# git fsck

Gitリポジトリのインデックス内にあるノードの妥当性と接続性を検証する。 注: 変更は行わない。

## Usage
- `git fsck` : 現在のリポジトリをチェックする
- `git fsck --tags` : 見つかったすべてのタグを一覧表示する
- `git fsck --root` : 見つかったすべてのルートノードを一覧表示する
- `git fsck --dangling --no-reflogs --unreachable --full` : reflog を参照せずに完全な整合性チェックを実行し、到達不能なオブジェクトと dangling オブジェクトをすべて表示する
- `git fsck --connectivity-only` : 接続性だけをチェックする (オブジェクトの整合性検証はスキップする)

## Links
- More information: https://git-scm.com/docs/git-fsck
