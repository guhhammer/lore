# gemini

Gemini AI와 대화형 프롬프트 실행.

## Usage
- `gemini` : 대화형 채팅을 위한 REPL 세션 시작
- `echo "Summarize the history of Rome" | gemini -p -` : 다른 명령의 출력을 Gemini로 전달하고 즉시 종료
- `gemini -m 모델_이름` : 특정 모델 사용 (기본값: auto-gemini-3)
- `gemini -s` : 샌드박스 컨테이너 내부에서 실행
- `gemini -i "Give me an example of recursion in Python"` : 프롬프트를 실행한 뒤 대화형 모드 유지
- `gemini --approval-mode default|auto_edit|yolo|plan` : 도구 호출의 승인 모드 설정
- `gemini -r latest|인덱스_번호|세션_아이디` : 세션 재개 (기본값 "latest"; 인덱스 번호 또는 UUID 사용 가능)

## Links
- More information: https://geminicli.com/docs/cli/cli-reference/
