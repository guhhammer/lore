# git bisect

バグを混入したコミットを二分探索で見つける。 Gitはコミットグラフ内を自動的に前後に移動し、問題のあるコミットを段階的に絞り込む。

## Usage
- `git bisect start 不良コミット 正常コミット` : 既知のバグありコミットと既知の正常なコミット (通常は古いコミット) で区切られた範囲でbisectセッションを開始する
- `git bisect good|bad` : `git bisect` が選択した各コミットについて、テスト後に "bad" または "good" としてマークする
- `git bisect reset` : bisectセッションを終了し、以前のブランチに戻る
- `git bisect skip` : bisect中にコミットをスキップする (例: 別の問題によりテストが失敗するコミット)
- `git bisect start 不良コミット 正常コミット -- ディレクトリ/サブディレクトリ/ファイル_または_ディレクトリ` : 指定したファイルまたはディレクトリを変更したコミットだけを対象にしてbisectセッションを開始する
- `git bisect run ディレクトリ/サブディレクトリ/テストスクリプト 任意のスクリプト引数` : "good" なら0、"bad" なら非0で `exit` するテストスクリプトを使ってbisect処理を自動化する
- `git bisect log` : これまでに実行された内容のログを表示する
- `git bisect visualize` : 確認対象として残っている候補コミットを表示する

## Links
- More information: https://git-scm.com/docs/git-bisect
