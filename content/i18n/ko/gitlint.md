# gitlint

Git 커밋 메시지 린터는 커밋 메시지의 스타일을 확인.

## Usage
- `gitlint` : 마지막 커밋 메시지를 확인
- `gitlint --commits 단일_refspec_인수` : 린트에 대한 커밋 범위
- `gitlint --extra-path 경로/대상/디렉터리` : 추가 사용자 정의 규칙이 있는 디렉토리 또는 Python 모듈의 경로 표시
- `gitlint --target 경로/대상/대상_디렉터리` : 특정 CI 작업 시작
- `gitlint --msg-filename 경로/대상/파일` : commit-msg가 포함된 파일의 경로 표시
- `gitlint --staged` : 로컬 저장소에서 단계적 커밋 메타 정보를 읽음

## Links
- More information: https://jorisroovers.com/gitlint/
