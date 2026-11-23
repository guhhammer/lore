# kubectl

Kubernetes クラスタに対してコマンドを実行するためのコマンドラインインターフェイス。 `run` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : リソースに関する情報をより詳細に一覧表示する
- `kubectl label pods ポッド名 unhealthy=true` : 指定したポッドにラベル 'unhealthy' と値 'true' を付けて更新する
- `kubectl get all` : 異なるタイプのリソースを全て一覧表示する
- `kubectl top pods|nodes` : ノードまたはポッドのリソース (CPU/Memory/Storage) 使用量を表示する
- `kubectl cluster-info` : マスターとクラスタサービスのアドレスを表示する
- `kubectl explain pods.spec.containers` : 特定のフィールドの説明を表示する
- `kubectl logs ポッド名` : ポッドまたは指定したリソース内のコンテナのログを表示する
- `kubectl exec ポッド名 -- ls /` : 既存のポッドでコマンドを実行する

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
