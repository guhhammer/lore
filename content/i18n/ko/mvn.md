# mvn

Apache Maven: Java 기반 프로젝트를 빌드하고 관리.

## Usage
- `mvn compile` : 프로젝트 컴파일
- `mvn package` : 컴파일된 코드를 `jar`와 같은 배포 가능한 형식으로 패키지
- `mvn package -D skipTests` : 유닛 테스트를 건너뛰고 컴파일 및 패키지
- `mvn install` : 빌드된 패키지를 로컬 Maven 저장소에 설치 (컴파일 및 패키지 명령도 실행됨)
- `mvn clean` : 타겟 디렉터리에서 빌드 아티팩트 삭제
- `mvn clean package` : 클린 후 패키지 단계 실행
- `mvn clean -P 프로필 package` : 주어진 빌드 프로필로 코드를 클린 후 패키지
- `mvn exec:java -D exec.mainClass="com.example.Main" -D exec.args="인수1 인수2 ..."` : 메인 메서드를 가진 클래스를 실행

## Links
- More information: https://manned.org/mvn
