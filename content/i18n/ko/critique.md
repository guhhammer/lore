# critique

Git 변경 사항을 구문 강조와 단어 단위 차이점으로 검토.

## Usage
- `critique` : 작업 트리의 스테이징되지 않은 변경 사항 검토
- `critique --staged` : 스테이징된 변경 사항만 검토
- `critique 커밋` : 지정한 커밋에서 도입된 변경 사항 검토
- `critique 브랜치_1 브랜치_2` : 두 브랜치 간의 변경 사항 비교
- `critique origin/브랜치 HEAD` : 로컬 `HEAD`와 원격 추적 브랜치 간의 변경 사항 검토
- `critique --watch` : 파일이 변경될 때마다 자동으로 새로 고침하며 변경 사항 검토
- `critique --filter "src/**/*.ts"` : 지정한 `glob` 패턴과 일치하는 파일만 검토
- `critique review --agent claude` : 지정한 AI 에이전트를 사용하여 코드 리뷰 생성

## Links
- More information: https://github.com/remorses/critique#usage
