# sgpt

OpenAI의 GPT 모델로 구동되는 명령줄 생산성 도구.

## Usage
- `sgpt "태양의 질량"` : 검색 엔진으로 사용하여 태양의 질량을 묻기
- `sgpt --shell "현재 디렉토리의 모든 파일을 읽기 전용으로 설정"` : 쉘 명령 실행, 현재 디렉토리의 모든 파일에 `chmod 444` 적용
- `sgpt --code "Python을 사용하여 fizz buzz 문제 해결"` : 코드 생성, 클래식한 fizz buzz 문제 해결
- `sgpt --chat 세션_이름 "내가 좋아하는 숫자를 기억해 주세요: 4"` : 고유한 세션 이름으로 채팅 세션 시작
- `sgpt --repl 명령` : `REPL` (Read-eval-print loop) 세션 시작
- `sgpt --help` : 도움말 표시

## Links
- More information: https://github.com/TheR1D/shell_gpt#readme
