# op

1Password의 데스크탑 앱을 위한 공식 CLI.

## Usage
- `op signin` : 1Password 계정에 로그인
- `op vault list` : 모든 금고 나열
- `op item get 항목_이름 --format json` : 항목 세부 정보를 JSON 형식으로 출력
- `op item create --category 카테고리_이름` : 기본 금고에 카테고리를 지정하여 새 항목 생성
- `op read 비밀_참조` : 참조된 비밀을 `stdout`에 출력
- `op run -- 명령` : 내보낸 환경 변수에서 비밀 참조를 명령에 전달
- `op run --env-file 경로/대상/환경_파일.env -- 명령` : 환경 파일에서 비밀 참조를 명령에 전달
- `op inject --in-file 경로/대상/입력_파일 --out-file 경로/대상/출력_파일` : 파일에서 비밀 참조를 읽어 일반 텍스트 비밀을 파일에 저장

## Links
- More information: https://developer.1password.com/docs/cli/reference/
