# bun outdated

최신 버전이 존재하는 의존성 목록을 확인.

## Usage
- `bun outdated` : 현재 프로젝트의 모든 오래된 의존성 목록 출력
- `bun outdated 패키지` : 특정 패키지가 오래되었는지 확인
- `bun outdated "패턴"` : `glob` 패턴에 일치하는 오래된 의존성 목록 출력
- `bun outdated -F "작업공간_패턴"` : 특정 워크스페이스에 대한 오래된 의존성 목록 출력
- `bun outdated -r` : 모노레포의 모든 워크스페이스를 재귀적으로 검사

## Links
- More information: https://bun.com/docs/pm/cli/outdated
