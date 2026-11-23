# opencode

AI 코딩 에이전트. `auth`, `models`, `web`와 같은 일부 하위명령어는 별도의 사용법 문서를 가짐.

## Usage
- `opencode` : 대화형 TUI 시작
- `opencode -c` : 가장 최근 세션 이어서 실행
- `opencode run "메시지"` : 프롬프트를 직접 전달하여 비대화형으로 실행
- `opencode run -m 제공자/모델 --agent 에이전트_이름 "메시지"` : 특정 모델과 에이전트를 지정하여 실행
- `opencode models` : 설정된 제공자에서 사용 가능한 모델 목록 출력
- `opencode auth login` : 제공자 인증 및 로그인 관리
- `opencode serve -h 호스트명 -p 포트` : API 접근을 위한 헤드리스 서버 실행
- `opencode agent create` : 사용자 정의 설정으로 새로운 에이전트 생성

## Links
- More information: https://opencode.ai/docs/cli/
