# codex

由 OpenAI 驱动的终端自然语言代码助手。 读取并编辑当前目录中的文件以完成请求。

## Usage
- `codex` : 在当前目录中启动交互式 Codex 会话
- `codex "提示词"` : 使用提示词运行单个 Codex 命令
- `codex --sandbox workspace-write "提示词"` : 运行提示词，允许 Codex 编辑工作区中的文件
- `codex -m 模型名称 "提示词"` : 使用特定模型
- `codex --oss --local-provider lmstudio|ollama "提示词"` : 使用本地开源模型提供商
- `/status` : [交互式] 显示会话配置和词元使用情况
- `codex -h` : 显示帮助

## Links
- More information: https://developers.openai.com/codex/cli/reference
