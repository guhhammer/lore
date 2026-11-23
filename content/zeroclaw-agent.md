# zeroclaw agent

Start the AI agent loop for interacting with ZeroClaw.

## Usage
- `zeroclaw agent -m "Hello, ZeroClaw!"` : send a single message to the AI agent
- `zeroclaw agent` : start interactive chat mode
- `zeroclaw agent -m "Hello" -p anthropic` : send a message with a specific provider
- `zeroclaw agent -m "Hello" --model anthropic/claude-sonnet-4-20250514` : send a message with a specific model
- `zeroclaw agent -m "Hello" -t 0.5` : send a message with custom temperature
- `zeroclaw agent -m "Hello" --peripheral nucleo-f401re:/dev/ttyACM0` : send a message and attach a hardware peripheral
- `zeroclaw agent -h` : display help

## Links
- More information: https://github.com/zeroclaw-labs/zeroclaw#quick-start
- tldr page: https://tldr.inbrowser.app/pages/common/zeroclaw-agent
