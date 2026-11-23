# mvn site

`pom.xml` 파일에 정의된 정보를 기반으로 프로젝트 웹사이트를 생성하는 명령어.

## Usage
- `mvn site` : site 플러그인을 사용하여 프로젝트 사이트 생성
- `mvn site -pl 모듈_이름` : maven 프로젝트의 사이트 생성 (멀티모듈 빌드)
- `mvn clean site` : 기존 사이트 출력 정리 후 새롭게 생성
- `mvn site -D skipTests` : 사이트 생성 시 테스트 실행 생략
- `mvn site-deploy` : 사이트를 생성 후, 원격 서버에 배포

## Links
- More information: https://maven.apache.org/plugins/maven-site-plugin/usage.html
