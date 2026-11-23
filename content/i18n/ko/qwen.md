# qwen

Qwen3-Coder와 대화형 프롬프트를 실행.

## Usage
- `qwen` : 대화형 REPL 세션 시작
- `echo "Summarize the history of Rome" | qwen -p` : 다른 명령의 출력을 Qwen에 전달하여 처리한 후 종료
- `qwen -m 모델_이름` : 기본 모델 재정의 (기본값: qwen3-coder-max)
- `qwen -s` : 샌드박스 컨테이너에서 실행
- `qwen -i "Give me an example of recursion in Python"` : 프롬프트를 실행한 후 대화형 모드 유지
- `qwen -a` : 모든 파일을 컨텍스트에 포함
- `qwen --show-memory-usage` : 상태 표시줄에 메모리 사용량 표시

## Links
- More information: https://github.com/QwenLM/qwen-code
