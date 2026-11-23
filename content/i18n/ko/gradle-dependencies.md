# gradle dependencies

Gradle 프로젝트의 의존성 트리 표시.

## Usage
- `gradle dependencies` : 모든 의존성 표시
- `gradle dependencies --configuration implementation` : 특정 설정의 의존성 표시
- `gradle :하위_프로젝트:dependencies` : 특정 하위 프로젝트의 의존성 표시
- `gradle dependencies >경로/대상/의존성.txt` : 의존성 정보를 파일로 저장
