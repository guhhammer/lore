# typst

Typst 파일을 PDF로 컴파일. 참고: 출력 위치 지정은 선택 사항입니다.

## Usage
- `typst init "템플릿" 경로/대상/폴더` : 주어진 디렉터리에 템플릿(예: `@preview/charged-ieee`)을 사용하여 새로운 Typst 프로젝트 초기화
- `typst compile 경로/대상/소스.typ 경로/대상/출력.pdf` : typst 파일 컴파일
- `typst watch 경로/대상/소스.typ 경로/대상/출력.pdf` : typst 파일을 감시하고 변경 시 다시 컴파일
- `typst --font-path 경로/대상/폰트_디렉토리 fonts` : 시스템 및 주어진 디렉토리에서 발견 가능한 모든 글꼴 나열

## Links
- More information: https://manned.org/typst
