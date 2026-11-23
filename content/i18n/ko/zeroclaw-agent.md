# zeroclaw agent

ZeroClaw와 상호작용하기 위한 AI 에이전트 루프를 시작.

## Usage
- `zeroclaw agent -m "Hello, ZeroClaw!"` : AI 에이전트에 단일 메시지 전송
- `zeroclaw agent` : 대화형 채팅 모드 시작
- `zeroclaw agent -m "Hello" -p anthropic` : 지정한 provider를 사용해 메시지 전송
- `zeroclaw agent -m "Hello" --model anthropic/claude-sonnet-4-20250514` : 지정한 모델을 사용해 메시지 전송
- `zeroclaw agent -m "Hello" -t 0.5` : 사용자 지정 temperature 값으로 메시지 전송
- `zeroclaw agent -m "Hello" --peripheral nucleo-f401re:/dev/ttyACM0` : 메시지를 전송하고 하드웨어 주변장치를 연결
- `zeroclaw agent -h` : 도움말 표시

## Links
- More information: https://github.com/zeroclaw-labs/zeroclaw#quick-start
