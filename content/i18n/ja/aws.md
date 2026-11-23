# aws

アマゾンウェブサービスの公式 CLI ツールです。 `s3` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `aws configure wizard` : AWS コマンドラインを設定する
- `aws configure sso` : SSO を利用して AWS コマンドラインを設定する
- `aws sts get-caller-identity` : 操作呼び出しに使用した認証情報の取得（パーミッションのトラブルシューティングに使用します）
- `aws dynamodb list-tables --region us-east-1 --output yaml` : リージョン内の AWS リソースをリストアップし、YAML で出力する
- `aws iam create-user --cli-auto-prompt` : 自動プロンプトを使用してコマンドを補助する
- `aws dynamodb wizard 新しいテーブル` : AWS リソースの対話型ウィザードを取得する
- `aws dynamodb update-table --generate-cli-skeleton` : JSON CLI スケルトンを生成する（Infrastructure as Code に役立ちます）
- `aws コマンド help` : AWS コマンドのヘルプを参照する

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
