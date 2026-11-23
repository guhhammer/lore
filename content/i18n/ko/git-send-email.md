# git send-email

여러 개의 패치를 이메일로 전송. 패치는 파일, 디렉토리 또는 수정 목록으로 지정할 수 있습니다.

## Usage
- `git send-email -1` : 현재 브랜치에서 마지막 커밋을 대화형으로 전송
- `git send-email -1 커밋` : 지정된 커밋 전송
- `git send-email -10` : 현재 브랜치에서 여러 개의 커밋(예: 10개) 전송
- `git send-email -커밋_수 --compose` : 패치 시리즈에 대한 소개 이메일 메시지 전송
- `git send-email -커밋_수 --annotate` : 전송할 각 패치의 이메일 메시지 검토 및 편집

## Links
- More information: https://git-scm.com/docs/git-send-email
