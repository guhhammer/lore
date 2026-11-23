# zeroclaw

Fast, small, and fully autonomous AI assistant infrastructure. Some subcommands such as `onboard`, `models`, `service`, etc. have their own usage documentation.

## Usage
- `zeroclaw onboard --api-key api_key --provider openrouter|anthropic|openai|...` : initialize workspace and configuration (quick setup)
- `zeroclaw onboard --interactive` : run the full interactive onboarding wizard
- `zeroclaw agent -m "Hello, ZeroClaw!"` : send a single message to the AI agent
- `zeroclaw agent` : start interactive chat mode
- `zeroclaw gateway` : start the gateway server (default: 127.0.0.1:8080)
- `zeroclaw daemon` : start full autonomous runtime (gateway + channels + heartbeat)
- `zeroclaw status` : check system status
- `zeroclaw doctor` : run diagnostics

## See also
- openclaw

## Categories
system, documents, monitoring

## Links
- More information: https://github.com/zeroclaw-labs/zeroclaw#quick-start
- tldr page: https://tldr.inbrowser.app/pages/common/zeroclaw
