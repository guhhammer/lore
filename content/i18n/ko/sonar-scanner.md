# sonar-scanner

Maven, Gradle, Ant와 같은 빌드 도구를 사용하지 않는 SonarQube 프로젝트를 위한 일반 스캐너.

## Usage
- `sonar-scanner` : 프로젝트의 루트 디렉토리에 있는 `sonar-project.properties`라는 구성 파일로 프로젝트 스캔
- `sonar-scanner -D project.settings=myproject.properties` : `sonar-project.properties`가 아닌 다른 구성 파일을 사용하여 프로젝트 스캔
- `sonar-scanner -X` : 디버깅 정보 출력
- `sonar-scanner -h` : 도움말 표시
