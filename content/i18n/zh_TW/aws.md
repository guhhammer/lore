# aws

Amazon Web Services 官方的命令列介面工具。

## Usage
- `aws configure wizard` : 設定 AWS 命令列
- `aws configure sso` : 使用 SSO 設定 AWS 命令列
- `aws sts get-caller-identity` : 取得呼叫者身分（用於排除權限問題）
- `aws dynamodb list-tables --region 區域 --output yaml` : 列出某個區域中的 AWS Dynamodb 並以 YAML 輸出
- `aws iam create-user --cli-auto-prompt` : 使用自動提示來幫助執行命令，
- `aws dynamodb wizard 精靈名稱` : 取得 AWS 互動式精靈
- `aws dynamodb update-table --generate-cli-skeleton` : 產生 JSON CLI 骨架（對於基礎設施即程式碼有用）
- `aws AWS指令 help` : 查看 AWS 指令的說明

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
