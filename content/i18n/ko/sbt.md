# sbt

Scala 및 Java 프로젝트를 위한 빌드 도구.

## Usage
- `sbt` : REPL(대화형 셸) 시작
- `sbt new scala/hello-world.g8` : gitHub에 호스팅된 기존 Giter8 템플릿에서 새 Scala 프로젝트 생성
- `sbt test` : 모든 테스트 컴파일 및 실행
- `sbt clean` : `target` 디렉토리의 모든 생성된 파일 삭제
- `sbt compile` : `src/main/scala` 및 `src/main/java` 디렉토리의 주요 소스 컴파일
- `sbt -sbt-version 버전` : 지정된 버전의 sbt 사용
- `sbt -sbt-jar 경로` : 특정 jar 파일을 sbt 실행 파일로 사용
- `sbt -h` : 도움말 표시

## Links
- More information: https://www.scala-sbt.org/1.x/docs/
