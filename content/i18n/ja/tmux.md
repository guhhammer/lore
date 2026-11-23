# tmux

端末のマルチプレクサ。 ウィンドウやペインなどによる複数セッションを可能にします。

## Usage
- `tmux` : 新規セッションの開始
- `tmux new -s セッション名` : 新しい名前付きセッションを開始する
- `tmux ls` : 既存のセッションを一覧表示
- `tmux a` : 直近に使用したセッションにアタッチ
- `<Ctrl b><d>` : 現在のセッションからの切り離し（tmuxセッション内）
- `<Ctrl b><c>` : 新しいウィンドウを作成する（tmuxセッション内）
- `<Ctrl b><w>` : セッションとウィンドウの切り替え（tmuxセッション内）
- `tmux kill-session -t セッション名` : 名前を指定してセッションを終了させる

## Links
- More information: https://github.com/tmux/tmux
