# codex

由 OpenAI 驅動的終端自然語言程式碼助手。 讀取並編輯當前目錄中的檔案以完成請求。

## Usage
- `codex` : 在當前目錄中啟動互動式 Codex 會話
- `codex "提示詞"` : 使用提示詞執行單個 Codex 命令
- `codex --sandbox workspace-write "提示詞"` : 執行提示詞，允許 Codex 編輯工作區中的檔案
- `codex -m 模型名稱 "提示詞"` : 使用特定模型
- `codex --oss --local-provider lmstudio|ollama "提示詞"` : 使用本地開源模型提供商
- `/status` : [互動式] 顯示會話配置和詞元使用情況
- `codex -h` : 顯示說明

## Links
- More information: https://developers.openai.com/codex/cli/reference
